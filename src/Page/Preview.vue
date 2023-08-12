<script lang="ts" setup>
import { computed, onMounted, onUnmounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn, emit } from "@tauri-apps/api/event";
import { ImagePayload } from "../tauri_pack/pack.ts";
import { mdi } from "vuetify/iconsets/mdi";

const listenId = ref<null | UnlistenFn>(null);
const objUrl = ref("");
const name = ref("");
const description = ref("");
const width = ref(-1);
const height = ref(-1);

const loading = computed(() => {
  return objUrl.value == "" || name.value == "";
});

onMounted(async () => {
  listenId.value = await appWindow.listen<ImagePayload>(
    "preview-img",
    ({ event, payload }) => {
      console.log(payload);
      objUrl.value = payload.obj_url;
      name.value = payload.name;
      description.value = "";
      if (payload.description != null) {
        description.value = payload.description;
      }

      width.value = -1;
      if (payload.width != null) {
        width.value = payload.width;
      }

      height.value = -1;
      if (payload.height != null) {
        height.value = payload.height;
      }

      emit("loading", false);
    }
  );
});

onUnmounted(() => {
  if (listenId.value != null) {
    listenId.value();
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
          {{ description }}({{ width }} * {{ height }})
        </v-card-subtitle>
        <v-card-text class="align-center pa-2" style="flex-grow: 1">
          <v-row
            id="preview-area"
            align="center"
            align-content="center"
            class="fill-height ma-0"
            justify="center"
            no-gutters
          >
            <v-img
              :src="objUrl"
              alt="Image"
              class="fill-height align-center"
              contain
            >
            </v-img>
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
#preview-area {
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
</style>