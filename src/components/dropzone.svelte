<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import {
        type DragDropEvent,
        getCurrentWebview,
    } from "@tauri-apps/api/webview";

    interface Props {
        files: string[];
        onDrop?: (files: string[]) => void;
        onDragOver?: (event: Event<DragDropEvent>) => void;
        onDragLeave?: (event: Event<DragDropEvent>) => void;
    }

    let {
        files = $bindable(),
        onDrop,
        onDragOver,
        onDragLeave,
    }: Props = $props();

    $effect(() => {
        let unlisten: UnlistenFn;
        getCurrentWebview()
            .onDragDropEvent((event) => {
                if (event.payload.type === "over") {
                    if (onDragOver) {
                        onDragOver(event);
                    }
                } else if (event.payload.type === "drop") {
                    const filePaths = event.payload.paths;
                    files = filePaths;
                    if (onDrop) {
                        onDrop(filePaths);
                    }
                } else if (event.payload.type === "leave") {
                    if (onDragLeave) {
                        onDragLeave(event);
                    }
                }
            })
            .then((callback) => {
                unlisten = callback;
            });

        return () => {
            unlisten();
        };
    });
</script>

<div></div>
