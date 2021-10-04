#ifndef BTREE_H
#define BTREE_H

#define TRUE 1
#define FALSE 0

#define T 3
#define MIN_KEYS T - 1
#define MIN_CHILDRENS T
#define MAX_KEYS 2 * T - 1
#define MAX_CHILDRENS 2 * T

#define PREDECESSOR 1
#define SUCCESSOR 2

typedef struct BTreeNode {
	int number_of_keys;
	int keys[MAX_KEYS];
	struct BTreeNode* children[MAX_CHILDRENS];
	int leaf;
} BTreeNode;

typedef struct NodeLocation {
	struct BTreeNode* node;
	int position; 
} NodeLocation;

typedef struct ArrayResult {
	int* values;
	int length;
} ArrayResult;


BTreeNode* b_tree_node_create();
NodeLocation* b_tree_search(BTreeNode* node, int key);
BTreeNode* b_tree_insert(BTreeNode* node, int key);
void delete_key(BTreeNode* node, int key);
ArrayResult* inorder(BTreeNode* node);
void print_b_tree(BTreeNode* tree);

#endif
