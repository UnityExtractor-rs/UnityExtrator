<template>
  <v-list density="compact">
    <v-list-group v-for="(item, i) in property.items" :value="i" :key="i">
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
              :id="`menu-right-activator-${i}`"
              color="indigo-lighten-2"
              variant="flat"
            >
          </v-btn>
            <v-menu
              v-if="item.menuItems"
              :activator="`#menu-right-activator-${i}`"
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
            {{ item.desription }}
          </v-tooltip>
        </v-list-item>
      </template>
      <v-list-item
        v-for="(child, i) in item.childen"
        :key="i"
        :prepend-icon="child.icon"
        :title="child.name"
        :subtitle="child.desription"
        @click="child.onClick"
      >
        <v-tooltip v-if="child.desription" activator="parent" location="top">
          {{ child.desription }}
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