// https://adventofcode.com/2021/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DAY 2
#define INPUT_LENGTH 1000

struct Order
{
  enum Direction
  {
    FORWARD,
    DOWN,
    UP,
  } direction;
  int amount;
} parsedInput[INPUT_LENGTH];

void parseInput(char *const input)
{
  char *pch = strtok(input, " \n");
  int orderNum = 0;
  while (pch != NULL)
  {
    switch (pch[0])
    {
    case 'f':
      parsedInput[orderNum].direction = FORWARD;
      break;
    case 'd':
      parsedInput[orderNum].direction = DOWN;
      break;
    case 'u':
      parsedInput[orderNum].direction = UP;
      break;
    default:
      printf("Unknown direction: %s\n", pch);
      exit(EXIT_FAILURE);
    }

    pch = strtok(NULL, " \n");
    parsedInput[orderNum].amount = _atoi64(pch);

    pch = strtok(NULL, " \n");
    orderNum++;
  }
}

char const *const part1()
{
  static unsigned int horizontal = 0;
  static unsigned int depth = 0;

  for (unsigned int i = 0; i < INPUT_LENGTH; i++)
  {
    switch (parsedInput[i].direction)
    {
    case FORWARD:
      horizontal += parsedInput[i].amount;
      break;
    case DOWN:
      depth += parsedInput[i].amount;
      break;
    case UP:
      depth -= parsedInput[i].amount;
      break;
    }
  }
  char *const result = malloc(sizeof(char) * 50);
  sprintf(result, "X: %d, Y: %d; Result: %d", horizontal, depth, horizontal * depth);
  return result;
};

char const *const part2()
{
  static unsigned int horizontal = 0;
  static unsigned int depth = 0;
  static unsigned int aim = 0;

  for (unsigned int i = 0; i < INPUT_LENGTH; i++)
  {
    switch (parsedInput[i].direction)
    {
    case FORWARD:
      horizontal += parsedInput[i].amount;
      depth += parsedInput[i].amount * aim;
      break;
    case DOWN:
      aim += parsedInput[i].amount;
      break;
    case UP:
      aim -= parsedInput[i].amount;
      break;
    }
  }
  char *const result = malloc(sizeof(char) * 50);
  sprintf(result, "X: %d, Y: %d; Result: %d", horizontal, depth, horizontal * depth);
  return result;
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