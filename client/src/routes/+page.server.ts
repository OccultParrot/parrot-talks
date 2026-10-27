import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { BACKEND_URL } from '$app/env/private';

export interface Post {
    id: number;
    title: string;
    content: string;
    createdAt: Date;
}

export const load: PageServerLoad = async ({ fetch }) => {
    // At start we load the 10 most recent posts
    // Simulate fetching posts from an API
    const response = await fetch(`${BACKEND_URL}/posts?limit=10&page=1`);

    if (!response.ok) {
        error(response.status, 'Failed to load posts');
    }

    const data = await response.json();
    const posts: Post[] = data.map((post: any) => ({
        id: post.id,
        title: post.title,
        content: post.content,
        createdAt: new Date(post.createdAt)
    }));

    return { posts };
};