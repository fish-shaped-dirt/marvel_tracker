<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";

import home from "./assets/navigation/home.svg";
</script>

<template>
  <nav class="navigation-bar">
    <RouterLink to="/">
      <span class="material-symbols-rounded">home</span>
    </RouterLink>
    <RouterLink to="/">
      <span class="material-symbols-rounded">bookmark</span>
    </RouterLink>
    <RouterLink to="/">
      <span class="material-symbols-rounded">calendar_month</span>
    </RouterLink>
    <RouterLink to="/settings">
      <span class="material-symbols-rounded">settings</span>
    </RouterLink>
  </nav>

  <div class="container">
    <RouterView />
  </div>
</template>
