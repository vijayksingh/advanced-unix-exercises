#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <dirent.h>

void err_quit(const char *msg)
{
  fprintf(stderr, "%s\n", msg);
  exit(1);
}

void err_sys(const char *msg, const char *arg)
{
  fprintf(stderr, "%s: %s\n", msg, arg);
  fprintf(stderr, ": %s\n", strerror(errno));
  exit(1);
}

int main(int argc, char *argv[])
{
  DIR *dp;
  struct dirent *dirp;

  if (argc != 2)
    err_quit("usage: ls directory_name");

  if ((dp = opendir(argv[1])) == NULL)
    err_sys("can't open %s", argv[1]);

  while ((dirp = readdir(dp)) != NULL)
    printf("%s\n", dirp->d_name);

  closedir(dp);
  return 0;
}