// https://adventofcode.com/2021/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DAY 1
#define INPUT_LENGTH 2000

static int parsedInput[INPUT_LENGTH];

void parseInput(char const *const input)
{
  char const *curLine = input;
  unsigned int lineNum = 0;
  while (curLine)
  {
    char const *nextLine = strchr(curLine, '\n');
    int curLineLen = nextLine ? nextLine - curLine : strlen(curLine);
    char *const tempStr = malloc(curLineLen + 1);
    memcpy(tempStr, curLine, curLineLen);
    tempStr[curLineLen] = '\0';

    parsedInput[lineNum] = _atoi64(tempStr);

    free(tempStr);

    curLine = nextLine ? nextLine + 1 : NULL;
    lineNum++;
  }
}

char const *const part1()
{
  static const unsigned char MEASUREMENTS = 2;
  int result = 0;
  for (unsigned int i = MEASUREMENTS - 1; i < INPUT_LENGTH; i++)
  {
    if (parsedInput[i] > parsedInput[i - 1])
    {
      result++;
    }
  }
  char *const resultString = malloc(sizeof(char) * 10);
  sprintf(resultString, "%d", result);
  return resultString;
};
char const *const part2()
{
  static const unsigned char MEASUREMENTS = 3;
  int result = 0;
  for (unsigned int i = MEASUREMENTS; i < INPUT_LENGTH; i++)
  {
    int measurementWindowA = parsedInput[i - 3] + parsedInput[i - 2] + parsedInput[i - 1];
    int measurementWindowB = parsedInput[i - 2] + parsedInput[i - 1] + parsedInput[i - 0];

    if (measurementWindowB > measurementWindowA)
    {
      result++;
    }
  }
  char *const resultString = malloc(sizeof(char) * 10);
  sprintf(resultString, "%d", result);
  return resultString;
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