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
    class="prose prose-neutral max-w-none
        prose-headings:text-neutral-900
        prose-a:text-green-700 prose-a:no-underline hover:prose-a:underline
        prose-blockquote:border-green-500 prose-blockquote:text-neutral-600
        prose-code:rounded prose-code:bg-green-50 prose-code:px-1 prose-code:text-green-800
        prose-img:rounded-xl prose-pre:rounded-xl"
>
    {@html html}
</div>