# Event-Driven stories generator

Implementing [this architecture](https://serverlessland.com/blog/implementing-an-event-driven-serverless-story-generation-application-with-chatgpt-and-dall-e--aws-compute-blog).

- Is there a way to make the generation of the speech and images asynchronous with `waitForTaskToken`?

## Learnings

- For some reason, I'm unable to specify `polly.amazonaws.com` as `Principal` (using the `Service` keyword).

  - Very strange. It does not help that the documentation is not that great.

  - **Polly seem to be using the role of the "invoker" to carry out the actions**.

    - Weird model. I would rather it required me to do `iam:PassRole` or to have a `RoleArn` property at the CFN level.

- **Enabling delivery status logging for SNS is a must** if you want to have any kind of debugging abilities.

  - Sadly **you cannot configure this setting via CFN**. You have to either use the SDK or the CLI.

- Using the _TaskToken_ with the SFN native SDK integrations is a bit hard. I had to encode the _TaskToken_ into the S3 path to be able to retrieve it later on.

  - I could use the sync version of the text-to-speech synthesizes, but that would not be fun!
