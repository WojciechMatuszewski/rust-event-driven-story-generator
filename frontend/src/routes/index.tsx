import { AWSIoTProvider } from "@aws-amplify/pubsub";
import { component$, useVisibleTask$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Amplify, PubSub } from "aws-amplify";

export default component$(() => {
  useVisibleTask$(async ({ cleanup }) => {
    /**
     * These have to run here. Amplify does not seem to be configurable with SSR initialization.
     */
    Amplify.configure({
      Auth: {
        identityPoolId: import.meta.env.PUBLIC_AWS_COGNITO_IDENTITY_POOL_ID,
        region: import.meta.env.PUBLIC_AWS_COGNITO_REGION,
        identityPoolRegion: import.meta.env.PUBLIC_AWS_COGNITO_REGION,
        userPoolId: import.meta.env.PUBLIC_AWS_USER_POOL_ID,
        userPoolWebClientId: import.meta.env.PUBLIC_AWS_USER_POOL_WEB_CLIENT_ID
      }
    });

    Amplify.addPluggable(
      new AWSIoTProvider({
        aws_pubsub_region: import.meta.env.PUBLIC_AWS_PUBSUB_REGION,
        aws_pubsub_endpoint: import.meta.env.PUBLIC_AWS_PUBSUB_ENDPOINT
      })
    );

    const subscription = PubSub.subscribe("story/1").subscribe({
      complete: () => console.log("Done"),
      error: (error) => console.error(error),
      next: (data) => console.log("Message received", data)
    });

    cleanup(() => {
      subscription.unsubscribe();
    });
  });

  return <div>works</div>;
});

export const head: DocumentHead = {
  title: "Welcome to Qwik",
  meta: [
    {
      name: "description",
      content: "Qwik site description"
    }
  ]
};
