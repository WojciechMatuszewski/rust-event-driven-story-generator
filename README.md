# Event-Driven stories generator

Roughly implements [this architecture](https://serverlessland.com/blog/implementing-an-event-driven-serverless-story-generation-application-with-chatgpt-and-dall-e--aws-compute-blog).

The changes I've made

- Use a public jokes API instead of OpenAI paid API

- Use state machine for orchestration. The dream would be to get rid of the AWS Lambda functions but that was not possible

  - The biggest challenge was to make the _transcribe audio_ call asynchronous with _TaskToken_.

- Uses IOT Core MQTT WebSockets to push updates to the frontend.

## Learnings

- For some reason, I'm unable to specify `polly.amazonaws.com` as `Principal` (using the `Service` keyword).

  - Very strange. It does not help that the documentation is not that great.

  - **Polly seem to be using the role of the "invoker" to carry out the actions**.

    - Weird model. I would rather it required me to do `iam:PassRole` or to have a `RoleArn` property at the CFN level.

- **Enabling delivery status logging for SNS is a must** if you want to have any kind of debugging abilities.

  - Sadly **you cannot configure this setting via CFN**. You have to either use the SDK or the CLI.

- Using the _TaskToken_ with the SFN native SDK integrations is a bit hard. I had to encode the _TaskToken_ into the S3 path to be able to retrieve it later on.

  - I could use the sync version of the text-to-speech synthesizes, but that would not be fun!

- **I did not have to create any particular IOT Core resource for the WebSocket to work!**.

  - This is great as I find the implementation of IOT Core WebSockets much better than the APIGW one.

    - You do not have to manage the state.

    - You can subscribe to given topics, even multiple of them!

    - It works nicely with IAM.

- The interface to generate a _presigned S3 URL_ is a bit weird to me.

  - The initial part makes sense – you specify the key, object and any other attributes.

  - The second part is a bit weird. To get the URL, not the S3 URI, you have to turn the returned data into HTTP request.

    - That would not be weird if it did not require me to provide a body, which does not make sense for the `getObject` calls?

      - Maybe I'm wrong, IDK.

- When working with **APIGW mapping templates**, I **was unable to join two strings together**.

  - One value was coming from the input – `input.json("$.executionArn")`, the other was a plain string – `JOKE#`.

  - For some reason, most of my tries yielded `"JOKE#""EXECUTION_ARN"` rather than the correct string.

- Be **mindful of the MQTT topic names**. For example, you **cannot have a topic containing `#`**. If you do, the MQTT will close the socket.

- And as always, do not be me and **do not waste your time debugging stale API definitions. Please REMEMBER TO DEPLOY THE APIGW!**.

- I was pretty **surprised by how Qwik handles environment variables**. Using `import.meta.env` feels a bit off after years of using `process.env` on both frontend and backend.

  - The `import.meta.env` convention is present in Vite and since Qwik is using Vite, no wonder it uses the same convention.

  - While reading the docs about the `import.meta`, I've noticed that, **according to MDN, the spec does not list any properties on this object**. It is up to browser implementers to add some (usually `url` and `resolve`).

    - This saddens me a bit, because it will most likely lead to incompatibilities between different browsers down the line.
