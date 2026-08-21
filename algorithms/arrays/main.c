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

void insert(struct Array *ar, int ind, int elem) {
    if (ind>=0 && ind <= ar->length) 
    {
        for(int i=ar->length;i>ind;i--)
            ar->a[i] = ar->a[i-1];
        ar->a[ind]=elem;
        ar->length++;
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


    print(&ar);

    return 0;
}