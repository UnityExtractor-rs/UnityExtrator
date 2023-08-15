<script lang="ts" setup>
import { computed } from "vue";
import { BtnDefine, notNullOr } from "../tauri_pack/pack.ts";

const property = defineProps<{
  buttons: BtnDefine[];
}>();
const moreBtn: BtnDefine = {
  icon: "mdi-dots-vertical",
  onClick: () => {},
  tooltip: "More",
  id: "menu-active",
};
const firstThree = computed(() => {
  let data;
  if (property.buttons.length >= 3) {
    data = property.buttons.slice(0, 3);
  } else {
    data = property.buttons;
  }
  data.push(moreBtn);
  console.log(data);
  return data;
});

const remain = computed(() => {
  if (property.buttons.length > 3) {
    let data = property.buttons.slice(3);
    console.log(data);
    return data;
  } else {
    return [];
  }
});
</script>

<template>
  <v-row
    align="center"
    align-content="center"
    class="d-flex flex-row ma-1"
    justify="center"
    no-gutters
  >
    <v-btn
      v-for="btn in firstThree"
      :key="btn.id"
      :id="btn.id"
      :color="notNullOr(btn.color, 'indigo')"
      :variant="notNullOr(btn.variant, 'elevated')"
      class="align-center ma-1"
      @click="btn.onClick"
    >
      <v-icon :icon="notNullOr(btn.icon, 'mdi-button-pointer')"></v-icon>
      <v-tooltip v-if="btn.tooltip" activator="parent" location="bottom">
        {{ btn.tooltip }}
      </v-tooltip>
    </v-btn>
    <v-menu activator="#menu-active">
      <v-list>
        <v-list-item
          v-for="(btn, idx) in remain"
          :key="idx"
          :color="notNullOr(btn.color, 'indigo')"
          :value="idx"
          @click="btn.onClick"
        >
          <template v-slot:prepend>
            <v-icon :icon="notNullOr(btn.icon, 'mdi-button-pointer')"></v-icon>
          </template>
          <v-list-item-title>{{ btn.tooltip }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-row>
</template>

<style scoped>
</style>