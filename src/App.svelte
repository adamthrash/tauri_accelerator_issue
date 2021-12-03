<script lang="javascript">
  import Body from './components/ui/Body.svelte.md'
  export let name;
</script>

<main>
  <Body name={name}/>
</main>

<style lang="scss">
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>