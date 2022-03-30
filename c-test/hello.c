#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Vector
{
    void *items;
    size_t item_size;
    size_t length;
    size_t capacity;
} Vector;

Vector vector_new(size_t item_size)
{
    Vector self;
    self.items = NULL;
    self.item_size = item_size;
    self.length = 0;
    self.capacity = 0;
    return self;
}

size_t vector_reserve(Vector *self, size_t capacity)
{
    assert(capacity >= self->length);
    assert(capacity != 0);

    void *items;
    if (self->capacity == 0)
    {
        items = malloc(capacity * self->item_size);
    }
    else
    {
        items = realloc(self->items, capacity * self->item_size);
    }

    if (items == NULL)
    {
        puts("Error allocating vector");
        exit(1);
    }

    self->items = items;
    self->capacity = capacity;
    return capacity;
}

void vector_debug(Vector *self)
{
    puts("Vector {");
    printf("\titems: %p\n", self->items);
    printf("\titem_size: %zu\n", self->item_size);
    printf("\tlength: %zu\n", self->length);
    printf("\tcapacity: %zu\n", self->capacity);
    puts("}");
}

int main()
{
    Vector cool = vector_new(sizeof(int));
    vector_reserve(&cool, 2);
    vector_debug(&cool);
    vector_reserve(&cool, 1);
    vector_debug(&cool);
    return 0;
}

int read_file(int argc, char const *argv[])
{
    FILE *filePointer = fopen("test.son", "r");

    if (filePointer == NULL)
    {
        puts("Error opening file");
        return 1;
    }

    int character;

    while ((character = fgetc(filePointer)) != EOF)
    {
        putchar(character);
    }

    return 0;
}
