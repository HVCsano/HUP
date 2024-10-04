<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'
    import { get_rates, type Rates } from '../lib/rate'
    let from = ''
    let to = ''
    let vals: Rates
    let rates = get_rates()
    onMount(async () => {
        vals = JSON.parse(await invoke('get_exchange_rates'))
    })
    let switchSelects = () => {
        let temp_to = to
        to = from
        from = temp_to
    }
</script>

<div class="flex h-screen">
    <div class="m-auto bg-gray-900 p-4 rounded-xl">
        <h1 class="text-3xl text-white text-center mb-2">HUP Váltó</h1>
        <form class="flex flex-col gap-2">
            <select
                name="from"
                bind:value={from}
                class="bg-slate-700 border-solid border-blue-400 rounded-xl border-2 p-1 text-white"
            >
                {#each rates as rate}
                    {#if rate[0] !== to}
                        <option class="text-white" value={rate[0]}
                            >{rate[1]} - {rate[2]}</option
                        >
                    {/if}
                {/each}
            </select>
            <select
                name="to"
                bind:value={to}
                class="bg-slate-700 border-solid border-blue-400 rounded-xl border-2 p-1 text-white"
            >
                {#each rates as rate}
                    {#if rate[0] !== from}
                        <option class="text-white" value={rate[0]}
                            >{rate[1]} - {rate[2]}</option
                        >
                    {/if}
                {/each}
            </select>
            <button
                class="text-white bg-green-600 uppercase px-3 rounded-xl"
                type="submit">Váltás</button
            >
            <button
                class="text-white bg-yellow-600 uppercase px-3 rounded-xl"
                on:click={switchSelects}>Csere</button
            >
        </form>
    </div>
</div>
