#include "garden.h"
#include "gardenlib.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define MAX_M 150000
#define MAX_Q 2000

int N, M, P, Q;
int R[MAX_M][2];
int G[MAX_Q];
int solutions[MAX_Q];
int answers[MAX_Q];
int answer_count = 0;

void answer(int x) {
    if (answer_count >= Q) {
        printf("Incorrect.  Too many answers.\n");
        exit(0);
    }

    answers[answer_count++] = x;
}

void read_input() {
    scanf("%d %d %d", &N, &M, &P);

    for (int i = 0; i < M; i++)
        scanf("%d %d", &R[i][0], &R[i][1]);

    scanf("%d", &Q);

    for (int i = 0; i < Q; i++)
        scanf("%d", &G[i]);

    for (int i = 0; i < Q; i++)
        scanf("%d", &solutions[i]);
}

int main() {
    read_input();

    clock_t clock_begin = clock();
    count_routes(N, M, P, R, Q, G);
    clock_t clock_end = clock();

    double time_spent = (double)(clock_end - clock_begin) / CLOCKS_PER_SEC;
    printf("Time: %f seconds\n", time_spent);

    if (answer_count != Q) {
        printf("Incorrect.  Too few answers.\n");
        exit(0);
    }

    int correct = 1;
    for (int i = 0; i < Q; i++)
        if (answers[i] != solutions[i])
            correct = 0;

    if (correct)
        printf("Correct.\n");

    else {
        printf("Incorrect.\nExpected: ");
        for (int i = 0; i < Q; i++)
            printf("%d ", solutions[i]);

        printf("\nReturned: ");
        for (int i = 0; i < Q; i++)
            printf("%d ", answers[i]);
        printf("\n");
    }

    return 0;
}
