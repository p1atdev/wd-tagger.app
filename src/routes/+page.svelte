<script lang="ts">
  import {
    type BatchleInferenceArgs,
    type InferenceResult,
    commands,
  } from "@/bindings";
  import Dropzone from "@/components/dropzone.svelte";
  import ModelSelect from "@/components/modelSelect.svelte";
  import ProbabilitiesDisplay from "@/components/probabilitiesDisplay.svelte";
  import { getDefaultModel, getModels } from "@/lib/model";
  import { filterTagsByThreshold } from "@/lib/tags";
  import {
    type Probabilities,
    probabilitiesType,
  } from "@/schema/probabilities";
  import Icon from "@iconify/svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { type } from "arktype";
  import { slide } from "svelte/transition";

  let model = $state(getDefaultModel());
  let predictions = $state<InferenceResult[]>([]);
  let files = $state<string[]>([]);
  let isProcessing = $state(false);
  let errorMessage = $state("");

  const predict = async (image_paths: string[], repoId: string) => {
    const args: BatchleInferenceArgs = {
      model_args: {
        repo_id: repoId,
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

  const validateProbabilities = (
    probs: Partial<{ [key in string]: number }>,
  ): Probabilities => {
    const result = probabilitiesType(probs);
    if (result instanceof type.errors) {
      console.error(result);
      return {} satisfies Probabilities;
    }
    return result;
  };

  $effect(() => {
    console.log("files", files);
    isProcessing = true;
    predict(files, model.repoId).finally(() => {
      isProcessing = false;
    });
  });
</script>

<main class="relative w-full background dotted-bg">
  <div class="flex h-full w-full">
    <div
      class="grow h-full flex flex-col justify-center overflow-y-scroll overflow-x-hidden"
    >
      <Dropzone bind:files />

      {#if files.length === 0}
        <div class="flex flex-col items-center gap-y-4">
          <Icon class="size-16" icon="mdi:file-upload" />
          <p>Drop your images here</p>
        </div>
      {/if}

      <div class="relative flex flex-col items-center gap-y-8 pt-8">
        {#each files as file, i}
          <img
            class="max-h-[80vh] max-w-full object-contain rounded-sm px-8 hover:scale-105 duration-150 transition-all"
            src={convertFileSrc(file)}
            alt="image {i}"
          />
        {/each}
      </div>

      <div
        class="absolute top-0 self-center max-w-md rounded-b-md bg-blue-500/50 hover:bg-blue-500/90 duration-75 transition-all"
      >
        <ModelSelect choices={getModels()} bind:selected={model} />
      </div>
    </div>

    {#if isProcessing || predictions.length > 0}
      <div
        transition:slide={{ axis: "x", duration: 200 }}
        class="relative shrink-0 max-w-[40%] w-sm overflow-y-scroll overflow-x-hidden"
      >
        <div
          class="absolute top-0 left-0 mt-6 mr-6 right-0 rounded-md max-w-full flex flex-col bg-blue-500/80"
        >
          {errorMessage}
          {#if isProcessing}
            <div
              class="flex flex-col h-[100vh] justify-center text-center gap-y-4 pb-32"
            >
              <Icon class="animate-spin size-16 mx-auto" icon="mdi:loading" />
              <p class="text-sm text-center">Processing...</p>
            </div>
          {:else if predictions.length > 0}
            <div class="flex flex-col gap-y-6">
              <div>
                {#each predictions as prediction}
                  <section class="flex flex-col gap-y-4 my-8 px-6">
                    <div>
                      <h3 class="text-lg font-bold mb-1">Character tags</h3>
                      <ProbabilitiesDisplay
                        probabilities={filterTagsByThreshold(
                          validateProbabilities(prediction.character),
                          0.35,
                        )}
                      />
                    </div>
                    <div>
                      <h3 class="text-lg font-bold mb-1">General tags</h3>
                      <ProbabilitiesDisplay
                        probabilities={filterTagsByThreshold(
                          validateProbabilities(prediction.general),
                          0.35,
                        )}
                      />
                    </div>
                  </section>
                {/each}
              </div>
            </div>
          {:else}
            <div class="flex flex-col h-[100vh] justify-center text-center">
              <p class="mb-10">No data to show</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  .background {
    margin: 0;
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
</style>
