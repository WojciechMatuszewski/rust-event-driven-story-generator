import { AWSIoTProvider } from "@aws-amplify/pubsub";
import { component$, useSignal, useVisibleTask$ } from "@builder.io/qwik";
import { routeAction$ } from "@builder.io/qwik-city";
import { Amplify, PubSub } from "aws-amplify";

export default component$(() => {
  useVisibleTask$(() => {
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
  });

  return <RequestStory />;
});

export const useRequestStory = routeAction$(async (_, { fail }) => {
  const response = await fetch(import.meta.env.PUBLIC_STORY_API_URL, {
    method: "POST"
  });

  if (response.status != 202) {
    const data = await response.text();
    return fail(500, { message: `Failed to create a story: ${data}` });
  }

  const data = (await response.json()) as { id: string };
  return {
    failed: false,
    id: data.id
  };
});

type StoryData = {
  status: string;
  text: string | null;
  audio: string | null;
};

const RequestStory = component$(() => {
  const storyId = useSignal<string | null>(null);
  const storyData = useSignal<StoryData | null>(null);

  const requestStory = useRequestStory();

  useVisibleTask$(({ cleanup, track }) => {
    track(() => storyId.value);
    if (!storyId.value) {
      return;
    }

    const subscription = PubSub.subscribe(`story/${storyId.value}`).subscribe({
      complete: () => console.log("Done"),
      error: (error) => {
        console.log("error", error);
      },
      next: (data) => {
        storyData.value = data.value as StoryData;
      }
    });

    cleanup(() => {
      console.log("unsubscribing");
      subscription.unsubscribe();
    });
  });

  if (!storyId.value) {
    return (
      <button
        type="button"
        onClick$={async () => {
          const result = await requestStory.submit();
          if ("message" in result) {
            return;
          }

          if (result.value.failed) {
            return;
          }

          storyId.value = `${result.value.id}`;
        }}
      >
        Request a story
      </button>
    );
  }

  if (!storyData.value) {
    return <p>Waiting for the story creation</p>;
  }

  return (
    <div>
      <p>Status: {storyData.value.status}</p>
      {storyData.value.text && <p>Text: {storyData.value.text}</p>}
      {storyData.value.audio && (
        <audio controls={true}>
          <source src={storyData.value.audio} type="audio/mpeg" />
        </audio>
      )}
    </div>
  );
});
