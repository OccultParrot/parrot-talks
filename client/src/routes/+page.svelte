<script lang="ts">
    interface Post {
        id: number;
        title: string;
        content: string;
        createdAt: Date;
    }

    // At start we load the 10 most recent posts
    let posts: Post[] = []

    async function loadPosts(): Promise<Post[]>{
        // Simulate fetching posts from an API
        let response = await fetch('');
        let data = await response.json();
        posts = data.map((post: any) => ({
            id: post.id,
            title: post.title,
            content: post.content,
            createdAt: new Date(post.createdAt)
        }));

        return posts;
    }
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

{#await loadPosts()}
    <p>Loading Posts...</p>
{:then posts: Post[]}
    {#each posts as p (p.id)}
        {@render post(p)}
    {/each}
{:catch error}
    <p>Error loading posts: {error.message}</p>
    
{/await}