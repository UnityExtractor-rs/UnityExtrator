<script lang="ts" setup>
import { computed, onMounted, onUnmounted, ref } from "vue";
import {
  ImagePreview,
  lisenPreview,
  PreviewMode,
  PreviewUnLisen,
  TextPreview,
  unListenAll,
} from "../tauri_pack/preview";

const previewPayload = ref<null | ImagePreview | TextPreview>(null);
const previewMode = ref<null | PreviewMode>(null);
const unListenFn = ref<null | PreviewUnLisen>(null);
onMounted(async () => {
  unListenFn.value = await lisenPreview(previewPayload, previewMode);
});

onUnmounted(() => {
  if (unListenFn.value != null) {
    unListenAll(unListenFn.value);
  }
});

const loading = computed(() => {
  return previewMode.value == null || previewPayload.value == null;
});

const name = computed(() => {
  if (previewMode.value == PreviewMode.Image) {
    return (previewPayload.value as ImagePreview).name;
  } else if (previewMode.value == PreviewMode.Text) {
    return (previewPayload.value as TextPreview).name;
  } else {
    return "Nothing to show";
  }
});

const description = computed(() => {
  if (previewMode.value == PreviewMode.Image) {
    return (previewPayload.value as ImagePreview).description;
  } else if (previewMode.value == PreviewMode.Text) {
    return (previewPayload.value as TextPreview).description;
  } else {
    return null;
  }
});

const payload = computed(() => {
  if (previewMode.value == PreviewMode.Image) {
    return (previewPayload.value as ImagePreview).imageUrl;
  } else if (previewMode.value == PreviewMode.Text) {
    return (previewPayload.value as TextPreview).payload;
  } else {
    return "";
  }
});

const meta = computed(() => {
  if (previewMode.value == PreviewMode.Image) {
    let value = previewPayload.value as ImagePreview;
    return ["Texture2D", value.fromat, `(${value.width} * ${value.height})`];
  } else if (previewMode.value == PreviewMode.Text) {
    let value = previewPayload.value as TextPreview;
    return ["TextAsset", `length: ${value.payload.length}`];
  } else {
    return [];
  }
});
</script>

<template>
  <v-row
    align-content="center"
    class="pa-2 bg-amber-accent-1 h-screen"
    no-gutters
  >
    <v-col align-self="center" class="h-100">
      <v-card class="fill-height d-flex flex-column" rounded :loading="loading">
        <v-card-title>{{ name }}</v-card-title>
        <v-card-subtitle>
          {{ description }} | {{ meta.join(", ") }}
        </v-card-subtitle>
        <v-card-text class="align-center pa-2" style="flex-grow: 1">
          <v-row
            :id="
              previewMode == PreviewMode.Image
                ? 'img-preview-area'
                : 'text-preview-area'
            "
            align="center"
            align-content="center"
            class="fill-height ma-0"
            justify="center"
            no-gutters
          >
            <v-img
              v-if="previewMode == PreviewMode.Image"
              :src="payload"
              alt="Image"
              class="fill-height align-center"
              contain
            >
            </v-img>
            <textarea
              id="preview-text-area"
              readonly
              no-resize
              v-if="previewMode == PreviewMode.Text"
              class="fill-height w-100 align-top text-align-left pa-2 elevation-2 rounded bg-grey-lighten-3"
              :value="payload"
            >
            </textarea>
          </v-row>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped lang="scss">
$color: rgba(0, 0, 0, 0.25);
$size: 20px;
$pos: calc($size / 2);
#img-preview-area {
  background-image: linear-gradient(
      45deg,
      $color 25%,
      transparent 25%,
      transparent 75%,
      $color 75%
    ),
    linear-gradient(
      45deg,
      $color 25%,
      transparent 25%,
      transparent 75%,
      $color 75%
    );
  background-position: 0 0, $pos $pos;
  background-size: $size $size;
  background-repeat: repeat;
}
#preview-text-area {
  resize: none;
  border: none;
  outline: none;
  font-family: ui-monospace, ui-sans-serif;
}
</style>