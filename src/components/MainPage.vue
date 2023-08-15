<template>
  <v-container class="h-screen d-flex flex-column ma-0 pa-2">
    <top-btn-bar :buttons="buttons"></top-btn-bar>
    <v-card :loading="loading" class="pa-1 h-100">
      <div class="h-100 overflow-y-auto overflow-x-hidden">
        <expandable-list :items="listInfo"></expandable-list>
      </div>
    </v-card>
  </v-container>
</template>

<script lang="ts" setup>
import { dialog, fs } from "@tauri-apps/api";
import { ref } from "vue";
import { blobToBase64, BtnDefine } from "../tauri_pack/pack.ts";
import TopBtnBar from "./TopBtnBar.vue";
import ExpandableList from "./ExpandableList.vue";
import { openOneFile } from "../tauri_pack/pack";
import {
  loadUnityAsset,
  UnityAsset,
  unityAssetToExpandable,
} from "../tauri_pack/load_unity";
import { computed } from "vue";
import { onMounted } from "vue";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { onUnmounted } from "vue";

const unityAsset = ref<UnityAsset[]>([]);
const loading = ref(false);
const listInfo = computed(() =>
  unityAsset.value.map((u: UnityAsset) => unityAssetToExpandable(u))
);

const onSelectFile = async () => {
  let path = await dialog.open({
    title: "Select File",
    multiple: false,
    directory: false,
    recursive: false,
    filters: [{ name: "Image", extensions: ["png", "jpg", "jpeg"] }],
  });
  if (path == null || (typeof path == "object" && path.length == 0)) {
    return;
  }
  let filename = typeof path == "string" ? path : path[0];
  console.log(filename);
  let stream = await fs.readBinaryFile(filename);
  console.log(stream.length);
  let blob = new Blob([stream]);
  console.log(blob);

  // property.selectFile(URL.createObjectURL(blob))
  await PreviewImage({
    name: filename,
    obj_url: await blobToBase64(blob),
    description: "Image from File",
  });
};

const buttons: BtnDefine[] = [
  {
    icon: "mdi-file-image-outline",
    onClick: onSelectFile,
    tooltip: "Open an Image",
  },
  {
    icon: "mdi-unity",
    onClick: async () => {
      loading.value = true;
      let file = await openOneFile("Select a Unity Asset File");
      if (file) {
        let value = await loadUnityAsset(file);
        unityAsset.value.push(value);
        console.log(unityAsset.value);
      }
    },
    tooltip: "Load a Unity Assets",
  },
  {
    icon: "mdi-cog-outline",
    onClick: () => {},
    tooltip: "Open an Image",
  },
  {
    icon: "mdi-file-search-outline",
    onClick: () => {},
    tooltip: "Search",
  },
  {
    icon: "mdi-close-circle-outline",
    onClick: () => {},
    tooltip: "Quit",
  },
];

const unliten = ref<UnlistenFn | null>();
onMounted(async () => {
  unliten.value = await listen<boolean>(
    "loading",
    ({ payload }: { payload: boolean }) => {
      loading.value = payload;
    }
  );
});

onUnmounted(() => {
  if (unliten.value) {
    unliten.value();
  }
});
</script>

<style scoped>
.scroll {
  overflow: scroll;
}

.v-card-item__content {
  align-self: start;
}
</style>