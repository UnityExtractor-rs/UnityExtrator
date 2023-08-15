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
import { ref } from "vue";
import { BtnDefine } from "../tauri_pack/pack.ts";
import TopBtnBar from "./TopBtnBar.vue";
import ExpandableList from "./ExpandableList.vue";
import { openOneFile } from "../tauri_pack/pack";
import {
  loadUnityAsset,
  syncLoadedAsset,
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

const buttons: BtnDefine[] = [
  {
    icon: "mdi-sync",
    onClick: async () => {
      let value = await syncLoadedAsset();
      unityAsset.value = value;
    },
    tooltip: "Sync Loaded Unity File",
  },
  {
    icon: "mdi-unity",
    onClick: async () => {
      loading.value = true;
      let file = await openOneFile("Select a Unity Asset File");
      if (file) {
        try {
          let value = await loadUnityAsset(file);
          unityAsset.value.push(value);
          console.log(unityAsset.value);
        } catch (error) {
          alert(`Load Unity Asset Failure: ${error}`);
        }
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