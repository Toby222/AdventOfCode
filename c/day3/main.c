// https://adventofcode.com/2021/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DAY 3
#define INPUT_LENGTH 1000

#define INPUT_BIT_SIZE 12

unsigned int parsedInput[INPUT_LENGTH];

void parseInput(char *const input)
{
  char const *curLine = input;
  unsigned int lineNum = 0;
  while (lineNum < INPUT_LENGTH)
  {
    char const *nextLine = strchr(curLine, '\n');
    int curLineLen = nextLine ? nextLine - curLine : strlen(curLine);
    char *const tempStr = malloc(curLineLen + 1);
    memcpy(tempStr, curLine, curLineLen);
    tempStr[curLineLen] = '\0';

    parsedInput[lineNum] = strtol(tempStr, NULL, 2);

    free(tempStr);

    curLine = nextLine ? nextLine + 1 : NULL;
    lineNum++;
  }
}

char const *const part1()
{
  unsigned int ones[INPUT_BIT_SIZE] = {0};

  for (unsigned int input_i = 0; input_i < INPUT_LENGTH; input_i++)
  {
    for (unsigned char i = 0; i < INPUT_BIT_SIZE; i++)
    {
      ones[i] += (parsedInput[input_i] >> i) & 1;
    }
  }

  char gammaStr[INPUT_BIT_SIZE + 1];
  sprintf(gammaStr, "%c%c%c%c%c%c%c%c%c%c%c%c",
          ones[INPUT_BIT_SIZE - 1] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 2] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 3] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 4] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 5] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 6] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 7] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 8] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 9] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 10] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 11] > INPUT_LENGTH / 2 ? '1' : '0',
          ones[INPUT_BIT_SIZE - 12] > INPUT_LENGTH / 2 ? '1' : '0');

  char epsilonStr[INPUT_BIT_SIZE + 1];
  sprintf(epsilonStr, "%c%c%c%c%c%c%c%c%c%c%c%c",
          ones[INPUT_BIT_SIZE - 1] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 2] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 3] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 4] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 5] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 6] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 7] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 8] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 9] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 10] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 11] > INPUT_LENGTH / 2 ? '0' : '1',
          ones[INPUT_BIT_SIZE - 12] > INPUT_LENGTH / 2 ? '0' : '1');

  long int gamma = strtol(gammaStr, NULL, 2);
  long int epsilon = strtol(epsilonStr, NULL, 2);

  char *const result = malloc(32);
  sprintf(result, "%ld", gamma * epsilon);
  return result;
};

char const *const part2()
{
  return "UNSOLVED";
};

int main(void)
{
  char *const fileName = malloc(sizeof(char) * sizeof("dayXX/input.txt"));
  sprintf(fileName, "day%d/input.txt", DAY);
  FILE *inputFile = fopen(fileName, "r");

  if (inputFile == NULL)
  {
    printf("Failed to open input file %s.\n", fileName);
    return 1;
  }

  fseek(inputFile, 0, SEEK_END);
  long length = ftell(inputFile);
  fseek(inputFile, 0, SEEK_SET);
  char *const input = malloc(length);
  if (input != NULL)
  {
    fread(input, 1, length, inputFile);
  }

  fclose(inputFile);

  parseInput(input);

  printf("Day %d\n", DAY);
  printf("Part 1:\n%s\n", part1());
  printf("--------------\n");
  printf("Part 2:\n%s\n\n", part2());
  return 0;
}
