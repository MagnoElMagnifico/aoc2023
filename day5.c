#include <stdio.h>

#define LINE_SIZE 300
#define MAX_SEEDS 100

void fill(int* buff, unsigned long size, int value) {
    for (unsigned int i = 0; i < size; i++) {
         buff[i] = value;
    }
}

int main(int argc, char** argv) {
    if (argc != 2) {
       printf("USO: %s <archivo>\n", argv[0]);
       return 0;
    }

    FILE* archivo = fopen(argv[1], "r");

    char line[LINE_SIZE];
    long int seeds[MAX_SEEDS];
    int uses[MAX_SEEDS];

    //// Read seeds ////
    fgets(line, LINE_SIZE, archivo);

    int pos = 7; // Ignore "seeds: "
    int bytes_read = 0;
    int seed_num = 0;
    while (sscanf(line+pos, " %ld %n", &seeds[seed_num], &bytes_read) == 1) {
        pos += bytes_read;
        seed_num++;
    }

    fill(uses, seed_num, 0);

    //// Process ////
    while (!feof(archivo)) {
        fgets(line, LINE_SIZE, archivo);

        if (line[0] >= '0' && line[0] <= '9') {

            // Read values of the line
            int output = 0;
            int source = 0;
            int step = 0;
            sscanf(line, " %d %d %d ", &output, &source, &step);

            for (int i = 0; i < seed_num; i++) {
                if (uses[i] > 0) {
                    continue;
                }

                if (seeds[i] >= source && seeds[i] <= source + step) {
                    seeds[i] = output + seeds[i]-source;
                    uses[i] = 1;
                }
            }
        } else {
            fill(uses, seed_num, 0);
        }
    }

    //// Find minimum ////
    long int min = seeds[0];

    for (int i = 1; i < seed_num; i++) {
        if (seeds[i] < min) {
            min = seeds[i];
        }
    }

    printf("Result: %ld\n", min);
    fclose(archivo);
}
