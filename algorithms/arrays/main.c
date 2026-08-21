#include <stdio.h>
#include <stdlib.h>

struct Array {
    int *a;
    int length;
    int capacity;
};

void initialize(struct Array *ar, int len, int cap) {
    ar->a = (int *)malloc(cap * sizeof(int));
    ar->length = len;
    ar->capacity = cap;
}

void print(struct Array *ar) {
    printf("elements of the array: \n");
    for (int i=0;i<ar->length;i++) {
        printf("%d\n", ar->a[i]);
    }
}

void append(struct Array *ar, int elem) {
    if (ar->length < ar->capacity)
        ar->a[ar->length] = elem;    
    ar->length++;
}

void insert(struct Array *ar, int index, int elem) {
    // check if index > 0 & index < ar.length - 1
    if (index >= 0 && index < ar->length) {
        for (int i=ar->length;i>index;i--) {
            ar->a[i] = ar->a[i-1];
        }
        ar->a[index] = elem;
        ar->length++;
    }
}

void delete(struct Array *ar, int index) {
    // check if index > 0 & index < ar.length - 1
    if (index >= 0 && index < ar->length) {
        for (int i=index;i<ar->length;i++) {
            ar->a[i] = ar->a[i+1];
        }
        ar->length--;
    }
}

int main()
{
    struct Array ar;
    initialize(&ar, 0, 10);

    append(&ar, 10);
    append(&ar, 20);
    append(&ar, 30);
    append(&ar, 40);
    append(&ar, 50);

    insert(&ar, 2, 45);
    printf("before deletion\n");
    print(&ar);
    
    delete(&ar, 2);
    
    
    printf("after deletion\n");
    print(&ar);

    return 0;
}