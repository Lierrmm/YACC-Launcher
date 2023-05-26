<!-- svelte-ignore a11y-click-events-have-key-events -->
<script>
  // @ts-nocheck
  import { appWindow } from "@tauri-apps/api/window";
  import "iconify-icon";

  let icons = [
    {
      icon: "fluent:minimize-16-filled",
      width: 28,
      height: 28,
      action: appWindow.minimize,
    },
    {
      icon: "iconamoon:close-fill",
      width: 28,
      height: 28,
      action: appWindow.close,
    },
  ];

  let menuItems = [
    {
      path: "/",
      text: "PLAY",
    },
    // {
    //   path: "/changelog",
    //   text: "CHANGELOG",
    // },
    {
      path: "/settings",
      text: "SETTINGS",
    },
  ];
</script>

<section class="h-16 titlebar rounded-tl-md rounded-tr-md">
  <!-- Menu Items -->
  <div class="flex justify-between bg-transparent" data-tauri-drag-region>
    <div class="flex justify-evenly rounded-tl-md w-auto">
      <ul>
        {#each menuItems as item}
          <li class="ml-4 mt-2 pr-6 inline-flex">
            <a
              href={item.path}
              class="text-gray-300 font-semibold text-xl hover:text-white duration-200"
              >{item.text}</a
            >
          </li>
        {/each}
      </ul>
    </div>

    <div class="h-12 float-right w-28 flex justify-evenly rounded-tr-md">
      {#each icons as icon}
        <div class="mt-3" on:click={() => icon.action()}>
          <iconify-icon
            class="text-white hover:text-gray-300 duration-200 cursor-pointer"
            icon={icon.icon}
            width={icon.width}
            height={icon.height}
          />
        </div>
      {/each}
    </div>
  </div>
</section>
