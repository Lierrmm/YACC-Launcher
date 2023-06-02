<script lang="ts">
  import Titlebar from "./components/TitleBar.svelte";
  import { Route } from "tinro";
  import Play from "./pages/Play.svelte";
  import Settings from "./pages/Settings.svelte";
</script>

<main class="h-screen rounded-md">
  <Titlebar />
  <Route path="/"><Play /></Route>
  <Route path="/settings"><Settings /></Route>
</main>
