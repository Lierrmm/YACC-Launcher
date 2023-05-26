<script lang="ts">
  import "iconify-icon";
  let path: string = "C:/FakePath/ForNow";
</script>

<section>
  <div class="ml-12 w-auto">
    <div>
      <h1 class="text-xl text-white font-bold">MANAGE INSTALLATION</h1>
    </div>

    <div class="mt-2 flex">
      <span
        class="inline-flex items-center px-3 [height:_2.4rem] text-sm text-gray-900 bg-gray-200 border border-r-0 border-gray-300 rounded-l"
      >
        <iconify-icon icon="material-symbols:folder" height="12" width="12" />
      </span>
      <input
        type="text"
        class="mb-6 bg-gray-100 border border-gray-300 text-gray-900 text-sm rounded-r block w-1/3 p-2"
        value={path}
        disabled
      />
    </div>
  </div>
</section>
