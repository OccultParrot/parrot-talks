<script lang="ts">
    import type { PageProps } from './$types';
    import type { Post } from './+page';

    let { data }: PageProps = $props();
</script>

{#snippet post(post: Post)}
    <div>
        <!-- Title Bar -->
        <div>
            <h2>{post.title}</h2>
            <small>{post.createdAt.toLocaleDateString()}</small>
        </div>
        <p>{post.content}</p>
    </div>
{/snippet}

{#each data.posts as p (p.id)}
    {@render post(p)}
{/each}