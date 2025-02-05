<script lang="ts">
  import {
    type BatchleInferenceArgs,
    type InferenceResult,
    commands,
  } from "@/bindings";
  import Dropzone from "@/components/dropzone.svelte";
  import Icon from "@iconify/svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";

  let predictions = $state<InferenceResult[]>([]);
  let files = $state<string[]>([]);
  let isProcessing = $state(false);
  let errorMessage = $state("");

  const predict = async (image_paths: string[]) => {
    const args: BatchleInferenceArgs = {
      model_args: {
        repo_id: "SmilingWolf/wd-swinv2-tagger-v3",
        model_file: "model.onnx",
        config_file: "config.json",
        tag_csv_file: "selected_tags.csv",
      },
      image_paths: image_paths,
    };

    const result = await commands.inferenceBatchImages(args);
    predictions = [];
    errorMessage = "";
    switch (result.status) {
      case "ok": {
        predictions = result.data;
        break;
      }
      case "error": {
        console.error(result.error);
        errorMessage = result.error.toString();
        break;
      }
    }
  };

  const onFilesSelected = async (files: string[]) => {
    console.log(files);
    isProcessing = true;
    await predict(files);
    isProcessing = false;
  };
</script>

<main class="relative w-full background dotted-bg">
  <div class="flex h-full w-full">
    <div
      class="grow h-full flex flex-col justify-center overflow-y-scroll overflow-x-hidden"
    >
      <Dropzone
        bind:files
        onDrop={(files) => {
          onFilesSelected(files);
        }}
      />

      {#if files.length === 0}
        <div class="flex flex-col items-center gap-y-4">
          <Icon class="size-16" icon="mdi:file-upload" />
          <p>Drop your images here</p>
        </div>
      {/if}

      <div class="flex flex-col items-center gap-y-8 pt-8 relative">
        {#each files as file, i}
          <img
            class="w-[50%] rounded-sm hover:scale-105 duration-150 transition-all"
            src={convertFileSrc(file)}
            alt="image {i}"
          />
        {/each}
      </div>
    </div>

    <div
      class="flex-none top-0 right-0 max-w-[33%] w-sm h-full overflow-y-scroll"
    >
      <div class="h-full mt-6 mr-6 bg-blue-500 opacity-80 rounded-md">
        <div class="max-w-full h-full">
          {#if isProcessing}
            <div class="pt-32 flex flex-col gap-y-4">
              <Icon class="animate-spin size-16 mx-auto" icon="mdi:loading" />
              <p class="text-sm">Processing...</p>
            </div>
          {/if}

          {errorMessage}

          {#each predictions as prediction}
            <div>
              {JSON.stringify(prediction)}
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</main>

<style>
  .background {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    height: 100vh;
  }
</style>
