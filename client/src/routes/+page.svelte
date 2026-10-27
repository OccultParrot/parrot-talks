<script lang="ts">
    import Markdown from '../components/Markdown.svelte';
    import type { PageProps } from './$types';
    import type { Post } from './+page';

    let { data }: PageProps = $props();

    const formatDate = (date: Date) =>
        date.toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' });
</script>

{#snippet post(post: Post)}
    <article class="rounded-2xl border border-neutral-200 p-6 shadow-sm transition hover:border-green-300 hover:shadow-md sm:p-8">
        <header class="mb-6 border-b border-green-100 pb-4">
            <h2 class="text-3xl font-bold tracking-tight text-green-700">{post.title}</h2>
            <time
                datetime={post.createdAt.toISOString()}
                class="mt-3 inline-block rounded-full bg-green-50 px-3 py-1 text-xs font-medium text-green-700"
            >
                {formatDate(post.createdAt)}
            </time>
        </header>
        <Markdown content={post.content} />
    </article>
{/snippet}

<div class="flex flex-col gap-8">
    {#each data.posts as p (p.id)}
        {@render post(p)}
    {:else}
        <p class="rounded-2xl border border-dashed border-green-200 bg-green-50/50 py-12 text-center text-green-800">
            No posts yet. Check back soon.
        </p>
    {/each}
</div>