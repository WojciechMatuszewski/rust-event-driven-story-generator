import { defineConfig } from "vite";
import { qwikVite } from "@builder.io/qwik/optimizer";
import { qwikCity } from "@builder.io/qwik-city/vite";
import tsconfigPaths from "vite-tsconfig-paths";
import { nodePolyfills } from "vite-plugin-node-polyfills";

export default defineConfig(() => {
  return {
    plugins: [
      qwikCity(),
      qwikVite(),
      tsconfigPaths(),
      /**
       * Required for the MQTT stuff to work
       */
      nodePolyfills({
        protocolImports: true
      })
    ],
    preview: {
      headers: {
        "Cache-Control": "public, max-age=600"
      }
    }
  };
});
