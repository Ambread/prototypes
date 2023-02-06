import { Component } from 'solid-js';
import {
    DragDropProvider,
    DragDropSensors,
    DragOverlay,
    SortableProvider,
    createSortable,
    closestCenter,
    useDragDropContext,
    DragEventHandler,
    Id,
} from '@thisbeyond/solid-dnd';
import { createSignal, For } from 'solid-js';

declare module 'solid-js' {
    namespace JSX {
        interface Directives {
            sortable: unknown;
        }
    }
}

const Sortable: Component<{ item: Id }> = (props) => {
    const sortable = createSortable(props.item);
    const context = useDragDropContext();
    return (
        <div
            use:sortable
            class="bg-blue-500 p-5 m-5"
            classList={{
                'opacity-25': sortable.isActiveDraggable,
                'transition-transform': !!context?.[0].active.draggable,
            }}
        >
            {props.item}
        </div>
    );
};

export const SortableVerticalListExample = () => {
    const [items, setItems] = createSignal<Id[]>([1, 2, 3]);
    const [activeItem, setActiveItem] = createSignal<Id>();

    const onDragStart: DragEventHandler = ({ draggable }) => {
        setActiveItem(draggable.id);
    };

    const onDragEnd: DragEventHandler = ({ draggable, droppable }) => {
        if (draggable && droppable) {
            const fromIndex = items().indexOf(draggable.id);
            const toIndex = items().indexOf(droppable.id);
            if (fromIndex !== toIndex) {
                const updatedItems = items().slice();
                const element = updatedItems.splice(fromIndex, 1);
                updatedItems.splice(toIndex, 0, ...element);
                setItems(updatedItems);
            }
        }
    };

    return (
        <DragDropProvider
            onDragStart={onDragStart}
            onDragEnd={onDragEnd}
            collisionDetector={closestCenter}
        >
            <DragDropSensors />
            <div class="flex flex-col">
                <SortableProvider ids={items()}>
                    <For each={items()}>
                        {(item) => <Sortable item={item} />}
                    </For>
                </SortableProvider>
            </div>
            <DragOverlay>
                <div class="bg-blue-500 p-5 m-5">{activeItem()}</div>
            </DragOverlay>
        </DragDropProvider>
    );
};

export const App: Component = () => {
    return (
        <div class="h-screen bg-slate-800 grid place-items-center">
            <SortableVerticalListExample />
        </div>
    );
};
