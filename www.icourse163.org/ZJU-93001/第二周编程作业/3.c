#include <stdio.h>
#include <stdlib.h>

struct Node
{
  int address;
  int data;
  struct Node* next;
};

void print_lst(struct Node* entry)
{
  int next;
  while(entry)
    {
      if (entry->next == NULL)
        {
          next = -1;
        }
      else
        {
          next = entry->next->address;
        }
      char nextfmt[6] = "-1";
      if (next != -1)
        {
          sprintf(nextfmt, "%05d", next);
        }
      printf("%05d %d %s\n", entry->address, entry->data, nextfmt);
      entry = entry->next;
    }
}

struct Node* reverse(struct Node* entry, int total_len, int cycle)
{
  int remaining = total_len;
  struct Node *curr=entry, *next, *cycle_head=entry, *next_next, *original_cycle_tail, *ret=entry;
  if (remaining >= cycle)
    {
      for(int i=0; i < cycle-1; i++) {
        ret = ret->next;
      }
    }
  while(remaining >= cycle)
    {
      struct Node *p=cycle_head, *q=cycle_head->next, *r;
      if (q == NULL)
        {
          break;
        }
      for(int i=0; i < cycle-1; i++)
        {
          r = q->next;
          q->next = p;
          p = q;
          q = r;
        }
      cycle_head->next = q;
      cycle_head = q;
      remaining -= cycle;
    }
  return ret;
}

int main()
{
  int entry, len, cycle;
  scanf("%d %d %d", &entry, &len, &cycle);
  int tmp_store[100000][2];

  for(int i=0; i < len; i++)
    {
      int addr, data, next;
      scanf("%d %d %d", &addr, &data, &next);
      tmp_store[addr][0] = data;
      tmp_store[addr][1] = next;
    }
  // return 0;

  int curr_addr = entry;
  struct Node* lst = malloc(sizeof(struct Node));
  struct Node* curr_node = lst;
  while (1)
    {
      curr_node->data = tmp_store[curr_addr][0];
      curr_node->address = curr_addr;
      if (tmp_store[curr_addr][1] == -1)
        {
          curr_node->next = NULL;
          break;
        }
      curr_node->next = malloc(sizeof(struct Node));
      curr_node = curr_node->next;
      curr_addr = tmp_store[curr_addr][1];
    }
  lst = reverse(lst, len, cycle);
  print_lst(lst);
}
