<template>
  <v-list density="compact">
    <v-list-group
      v-for="(item, itemIdx) in property.items"
      :value="itemIdx"
      :key="itemIdx"
    >
      <template v-slot:activator="{ props }">
        <v-list-item
          v-bind="props"
          :title="item.name"
          :subtitle="item.desription"
        >
          <template v-slot:prepend>
            <v-btn
              size="small"
              :icon="item.icon"
              class="mr-2"
              color="indigo-lighten-2"
              variant="flat"
              :disabled="!item.menuItems"
            >
            </v-btn>
            <v-menu
              v-if="item.menuItems"
              activator="parent"
            >
              <v-list>
                <v-list-item
                  v-for="(menu, idx) in item.menuItems"
                  :key="idx"
                  :prepend-icon="menu.icon"
                  :title="menu.text"
                  @click="
                    () => {
                      menu.onClick(item);
                    }
                  "
                >
                </v-list-item>
              </v-list>
            </v-menu>
          </template>
          <v-tooltip v-if="item.desription" activator="parent" location="top">
            {{ `${item.name} - ${item.desription}` }}
          </v-tooltip>
        </v-list-item>
      </template>
      <v-list-item
        v-for="(child, childIdx) in item.childen"
        :key="childIdx"
        :title="child.name"
        :subtitle="child.desription"
        @click="child.onClick"
      >
        <template v-slot:prepend>
          <v-btn
            size="small"
            :icon="child.icon"
            class="mr-2"
            color="indigo-lighten-2"
            variant="flat"
            :disabled="!child.menuItems"
          >
          </v-btn>
          <v-menu v-if="child.menuItems" activator="parent">
            <v-list>
              <v-list-item
                v-for="(menu, idx) in child.menuItems"
                :key="idx"
                :prepend-icon="menu.icon"
                :title="menu.text"
                @click="
                  () => {
                    menu.onClick({ item, child });
                  }
                "
              >
              </v-list-item>
            </v-list>
          </v-menu>
        </template>
        <v-tooltip v-if="child.desription" activator="parent" location="top">
          {{ `${child.name} - ${child.desription}` }}
        </v-tooltip>
      </v-list-item>
    </v-list-group>
    <v-list-item type="divider"></v-list-item>
  </v-list>
</template>

<script setup lang="ts">
import { ExpandItem } from "../dto/expandable";

const property = defineProps<{
  items: ExpandItem[];
}>();
</script>

<style>
</style>