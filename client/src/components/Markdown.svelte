<script lang="ts">
	import { marked } from 'marked';
	import sanitizeHtml from 'sanitize-html';

	let { content }: { content: string } = $props();

	const html = $derived(
		sanitizeHtml(marked.parse(content, { async: false }), {
			allowedTags: sanitizeHtml.defaults.allowedTags.concat(['img']),
			allowedAttributes: {
				...sanitizeHtml.defaults.allowedAttributes,
				img: ['src', 'alt'],
				code: ['class']
			}
		})
	);
</script>

<div
	class="prose max-w-none prose-neutral
        prose-headings:text-green-700
        prose-a:text-green-700 prose-a:no-underline hover:prose-a:underline
        prose-blockquote:border-green-500 prose-blockquote:text-neutral-600
        prose-code:rounded
        prose-code:text-green-800 prose-pre:rounded-xl
        prose-pre:bg-green-50 prose-img:rounded-xl"
>
	{@html html}
</div>
