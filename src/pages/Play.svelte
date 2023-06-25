<script>
  import { YaccState } from "../utils/YaccState";
  import store from "../stores/stores";

  const launch = () =>
    !$store.game_path ? store.updateGamePath() : store.LaunchGame();

  const playButtonLabel = () => {
    if ($store.yacc_is_running) return "YACC is Running";

    switch ($store.yacc_state) {
      case YaccState.GAME_NOT_FOUND:
        return "Select Game Directory";
      case YaccState.INSTALL:
        return "Install YACC";
      case YaccState.INSTALLING:
        return "Installing...";
      case YaccState.MUST_UPDATE:
        return "Requires Update";
      case YaccState.UPDATING:
        return "Updating...";
      case YaccState.READY_TO_PLAY:
        return "Launch Game";

      default:
        return "";
    }
  };
</script>

<footer class="rounded-lg shadow m-4 ml-12 fixed bottom-8">
  <div>
    <h1 class="text-6xl text-white font-bold">YACC</h1>
    <p class="text-sm text-gray-500 mt-2">
      <!-- <a href="/changelog" class="underline">Read Patch Notes</a> -->
    </p>
  </div>

  <div
    class="w-full mx-auto max-w-screen-xl py-4 md:flex md:items-center md:justify-between"
  >
    <button
      type="button"
      on:click={() => launch()}
      class="text-white bg-blue-700 hover:bg-blue-800 font-medium rounded text-base px-6 py-3.5 text-center"
      >{playButtonLabel()}</button
    >
  </div>
</footer>
