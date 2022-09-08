<svelte:head>
  <title>cube solver</title>
</svelte:head>

<script>
  import { onMount } from 'svelte'
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from './assets/vite.svg'
  import wasmLogo from './assets/wasm.svg'
  import init, { solve } from '../pkg/'

  let scramble = "U";
  const ops = "URFU'R'F'";
  let solution_by_urf = ['(..)'];
  let solution_by_roux = ['(..)'];

  function run() {
    solution_by_urf = ['(..)'];
    solution_by_roux = ['(..)'];
    try {
      const max_depth = 7;
      const num = 5;
      let solution = solve("Scramble{" + scramble + "}", ops, max_depth, num, false);
      solution_by_urf = solution.split(';');
    } catch(e) {
      console.warn(e);
      solution_by_urf = ['Something Error (check Scramble is invalid?)'];
    }
    try {
      let solution = solve("Scramble{" + scramble + "}", "", 0, 0, true);
      solution_by_roux = solution.split(';');
    } catch(e) {
      console.warn(e);
      solution_by_roux = ['Something Error'];
    }
  };

  onMount(async () => {
    init();
    setTimeout(run, 500);
  });
</script>

<main>

  <section class="section">
    <div>
      <a href="https://vitejs.dev" target="_blank"> 
        <img src={viteLogo} class="logo" alt="Vite Logo" />
      </a>
      <a href="https://svelte.dev" target="_blank"> 
        <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
      </a>
      <a href="https://webassembly.org" target="_blank"> 
        <img src={wasmLogo} class="logo wasm" alt="WASM Logo" />
      </a>
    </div>
    <h1 class=title>cube solver</h1>
  </section>

  <section class="section">
    <div class="container">
      <div class="field has-addons">
        <div class="field-label is-normal">
          <label class="label" for="scramble">Scramble</label>
        </div>
        <div class="control">
          <input class="input" type="text" id="scramble" bind:value={scramble} />
        </div>
        <div class="control">
          <button class="button is-info" on:click={run}>Solve</button>
        </div>
      </div>
    </div>
  </section>

  <section class="section">
    <div class="table-container">

      <table class="table is-fullwidth">
        <thead>
          <tr><th>Solutions by URF</th></tr>
        </thead>
        <tbody>
          {#each solution_by_urf as sol}
            <tr><td>{sol}</td></tr>
          {/each}
        </tbody>
      </table>

      <table class="table is-fullwidth">
        <thead>
          <tr><th>Solution by Roux</th></tr>
        </thead>
        <tbody>
          {#each solution_by_roux as sol}
            <tr><td>{sol}</td></tr>
          {/each}
        </tbody>
      </table>

    </div>
  </section>

  <section class="section">
    <div class="content has-text-centered">
      <a href="https://sites.google.com/site/happybusy/" target="_blank">
        <img src="/cube/busy_banner.png" alt="happybusy" /></a>
    </div>
  </section>

</main>

<style>
  @import "../node_modules/bulma/css/bulma.css";

  img.logo {
    width: 2rem;
    height: 2rem;
  }

</style>
