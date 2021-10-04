#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <math.h>

#include "b_tree.h"


// ------------------------ CREATE B TREE ------------------------

BTreeNode* b_tree_node_create() {
	BTreeNode* root = (BTreeNode*) malloc(sizeof(BTreeNode));
	root->leaf = TRUE;
	root->number_of_keys = 0;

	for(int i = 0; i < MAX_KEYS; i++) {
		root->keys[i] = 0;
		root->children[i] = NULL;
	}
	root->children[MAX_CHILDRENS - 1] = NULL;

	// Disk-Write(root)
	return root;
}


// ------------------------ SEARCH B TREE ------------------------

NodeLocation* b_tree_search(BTreeNode* node, int key) {
	int i = -1;
	while(i < node->number_of_keys && key > node->keys[++i]);
	
	NodeLocation* result = (NodeLocation *) malloc(sizeof(NodeLocation));
	if(i < node->number_of_keys && key == node->keys[i]) {
		result->node = node;
		result->position = i;
		
		return result;
	} else if (node->leaf) {
		result->node = NULL;
		result->position = -1;
		
		return result;
	}
	
	// Disk-Read(node->children[i])
	return b_tree_search(node->children[i], key);
}


// ------------------------ INSERT B TREE ------------------------

void b_tree_split_child(BTreeNode* node, int position) {
	BTreeNode* z = b_tree_node_create();
    BTreeNode* y = node->children[position];
	z->leaf = y->leaf;
	z->number_of_keys = MIN_KEYS;

	for(int i = 0; i < MIN_CHILDRENS - 1; i++) {
		z->keys[i] = y->keys[MIN_CHILDRENS + i];
	}
	if(!y->leaf) {
		for(int i = 0; i < MIN_CHILDRENS; i++) {
			z->children[i] = y->children[MIN_CHILDRENS + i];
		}
	}
	y->number_of_keys = MIN_KEYS;

	for(int i = node->number_of_keys; i > position; i--) {
		node->keys[i] = node->keys[i - 1];
		node->children[i + 1] = node->children[i];
	}
	node->children[position + 1] = z;
	node->keys[position] = y->keys[MIN_KEYS];
	node->number_of_keys += 1;

	//Disk-Write(y)
	//Disk-Write(z)
	//Disk-Write(node)
}

void b_tree_insert_nonfull(BTreeNode* node, int key) {
	int i = node->number_of_keys - 1;
	if(node->leaf) {
		while(i >= 0 && key < node->keys[i]) {
			node->keys[i + 1] = node->keys[i];
			i -= 1;
		}
		node->keys[i + 1] = key;
		node->number_of_keys += 1;
		// Disk-Write(node)
	} else {
		while(i >= 0 && key < node->keys[i]) {
			i -= 1;
		}
		i += 1;
		// Disk-Read(node->children)
		if(node->children[i]->number_of_keys == MAX_KEYS) {
			b_tree_split_child(node, i);
			if(key > node->keys[i]) {
				i += 1;
			}
		}
		b_tree_insert_nonfull(node->children[i], key);
	}
}

BTreeNode* b_tree_insert(BTreeNode* node, int key) {
	if(node->number_of_keys != MAX_KEYS) {
		b_tree_insert_nonfull(node, key);
		return node;
	}

	BTreeNode* s = b_tree_node_create();
	s->leaf = FALSE;
	s->number_of_keys = 0;
	s->children[0] = node;
	b_tree_split_child(s, 0);
	b_tree_insert_nonfull(s, key);
	
	return s;
}


// ------------------------ DELETE B TREE ------------------------

void delete_from_leaf(BTreeNode* leaf, int position) {
	for(int i = position; i < leaf->number_of_keys - 1; i++) {
		leaf->keys[i] = leaf->keys[i + 1];
	}
	leaf->number_of_keys -= 1;
}

void replace_key_with(int relation, BTreeNode* node, int position) {
	int relative_position = relation == 1 ? position : position + 1;
	BTreeNode* relative = node->children[relative_position];

	BTreeNode* children = relative;
	int key_position = relation == 1 ? children->number_of_keys - 1 : 0;
	int children_position = key_position == 0 ? 0 : key_position + 1;
	while(!children->leaf) {
		children = children->children[children_position];
		key_position = relation == 1 ? children->number_of_keys - 1 : 0;
		children_position = key_position == 0 ? 0 : key_position + 1;
	}
	
	int new_key = children->keys[key_position];
	node->keys[position] = new_key;

	delete_key(relative, new_key);
}

int valid_sib_position(BTreeNode* node, int position) {
    int is_the_last_children = position == node->number_of_keys; 
    return position + (is_the_last_children ? -1 : 1);
}

int min(int a, int b) {
	return a < b ? a : b;
}

void merge_sibs(BTreeNode* node, int position) {
	int sib_position = valid_sib_position(node, position);
	int little_sib_position = min(position, sib_position);
	int big_sib_position = little_sib_position + 1;
	BTreeNode* little_sib = node->children[little_sib_position];
	BTreeNode* big_sib = node->children[big_sib_position];

	int median_position = little_sib_position;
	little_sib->keys[MIN_KEYS] = node->keys[median_position];
	for(int i = 0; i < MIN_KEYS; i++) {
		little_sib->keys[MIN_KEYS + 1 + i] = big_sib->keys[i];
		little_sib->children[MIN_CHILDRENS + i] = big_sib->children[i];
	}
		little_sib->children[2 * MIN_CHILDRENS - 1] = big_sib->children[MIN_CHILDRENS - 1];
	little_sib->number_of_keys *= 2;
	little_sib->number_of_keys += 1;
	free(big_sib);

	for(int i = position; i < node->number_of_keys - 1; i++) {
		node->keys[i] = node->keys[i + 1];
		node->children[i + 1] = node->children[i + 2];
	}
	node->children[position] = little_sib;
	node->number_of_keys -= 1;

	if(node->number_of_keys == 0) *node = *node->children[0];
}

void delete_from_internal_node(BTreeNode* node, int key, int position) {
	BTreeNode* predecessor = node->children[position];
	BTreeNode* successor = node->children[position + 1];
	if(predecessor->number_of_keys > MIN_KEYS) {
		replace_key_with(PREDECESSOR, node, position);
	} else if(successor->number_of_keys > MIN_KEYS) {
		replace_key_with(SUCCESSOR, node, position);
	} else {
		merge_sibs(node, position);
		delete_key(node, key);
	}
}

int search_subtree_with_key(BTreeNode* node, int key) {
	int i = node->number_of_keys + 1;
	while(--i > -1 && b_tree_search(node->children[i], key)->position < 0);
	
	return i;
}

int sib_with_suficient_keys(BTreeNode* node, int position) {
	if(position > 0
	   && node->children[position - 1]->number_of_keys != MIN_KEYS) {
		return 1;
	}

	if(position < node->number_of_keys
	   && node->children[position + 1]->number_of_keys != MIN_KEYS) {
		return 2;
	}

	return 0;
}

void rotate_key_from_left_sib(BTreeNode* root, BTreeNode* subtree, int position) {
	for(int i = subtree->number_of_keys; i > 0; i--) {
		subtree->keys[i] = subtree->keys[i - 1];
		subtree->children[i + 1] = subtree->children[i];
	}
	subtree->keys[0] = root->keys[position - 1];
	subtree->children[1] = subtree->children[0];
	subtree->number_of_keys += 1;

	BTreeNode* sib = root->children[position - 1];
	subtree->children[0] = sib->children[sib->number_of_keys];

	root->keys[position - 1] = sib->keys[sib->number_of_keys - 1];
	sib->number_of_keys -= 1;
}

void rotate_key_from_right_sib(BTreeNode* root, BTreeNode* subtree, int position) {
	BTreeNode* sib = root->children[position + 1];

	subtree->keys[subtree->number_of_keys] = root->keys[position];
	subtree->children[subtree->number_of_keys + 1] = sib->children[0];
	subtree->number_of_keys += 1;

	root->keys[position] = sib->keys[0];

	for(int i = 0; i < sib->number_of_keys; i++) {
		sib->keys[i] = sib->keys[i + 1];
		sib->children[i] = sib->children[i + 1];
	}
	sib->children[sib->number_of_keys] = sib->children[sib->number_of_keys + 1];
	sib->number_of_keys -= 1;
}


int key_position_in_node(BTreeNode* node, int key) {
	int i = -1;
	while(i < node->number_of_keys && key > node->keys[++i]);
	
	NodeLocation* result = (NodeLocation *) malloc(sizeof(NodeLocation));
	if(i < node->number_of_keys && key == node->keys[i]) return i;

	return -1;
}


void delete_key(BTreeNode* node, int key) {
	int key_not_in_tree = b_tree_search(node, key)->position == -1;
	if(key_not_in_tree) return;

	int position = key_position_in_node(node, key);
	int key_in_node = position != -1;
	if(key_in_node && node->leaf) {
		delete_from_leaf(node, position);
		return;
	}

	if(key_in_node && !node->leaf) {
		delete_from_internal_node(node, key, position);
		return;
	}

	int subtree_position = search_subtree_with_key(node, key);
	BTreeNode* subtree = node->children[subtree_position];
	if(subtree->number_of_keys == MIN_KEYS) {
		int sibs_do_not_have_min_keys = sib_with_suficient_keys(node, subtree_position);
		if(!sibs_do_not_have_min_keys) {
			merge_sibs(node, subtree_position);
			delete_key(node, key);
			return;
		}

		if(sibs_do_not_have_min_keys == 1) {
			rotate_key_from_left_sib(node, subtree, subtree_position);
		} else {
			rotate_key_from_right_sib(node, subtree, subtree_position);
		}
	}

	delete_key(subtree, key);
}


// ------------------------ B TREE HELPERS ------------------------

ArrayResult* inorder(BTreeNode* node) {
	ArrayResult* result = (ArrayResult*) malloc(sizeof(ArrayResult));
	if(node->leaf) {
		result->values = (int*) malloc(node->number_of_keys * sizeof(int));
		for(int i = 0; i < node->number_of_keys; i++) {
			result->values[i] = node->keys[i];
		}
		result->length = node->number_of_keys;

		return result;
	}

	result->values = (int*) malloc(sizeof(int));
	result->length = 0;
	for(int i = 0; i < node->number_of_keys + 1; i++) {
		ArrayResult* intermediate_result = inorder(node->children[i]);
		result->values = (int*) realloc(result->values, (result->length + intermediate_result->length + 1) * sizeof(int));
		for(int j = 0; j < intermediate_result->length; j++) {
			result->values[result->length + j] = intermediate_result->values[j];
		}
		if(i < node->number_of_keys) {
			result->values[result->length + intermediate_result->length] = node->keys[i];
			result->length += 1;
		}
		result->length += intermediate_result->length;
	}

	return result;
}

void recursive_print(BTreeNode* tree, int depth) {
	for(int i = 0; i < tree->number_of_keys + 1; i++) {
		if(tree->children[i] != NULL) {
			printf("%*c", 2 * depth, ' ');
			printf("Children %i has keys = ", i);
			for(int j = 0; j < tree->children[i]->number_of_keys; j++) {
				printf("%i,", tree->children[i]->keys[j]);
			}
			printf("\n");

			if(!tree->leaf) {
				recursive_print(tree->children[i], depth + 1);
			}
			
			if(!tree->children[i]->leaf) {
				printf("\n");
			}
		}
	}
}

void print_b_tree(BTreeNode* tree) {
	printf("\nRoot has %i keys = ", tree->number_of_keys);
	for(int i = 0; i < tree->number_of_keys; i++) {
		printf("%i,", tree->keys[i]);
	}
	printf("\n");

	if(!tree->leaf) {
		recursive_print(tree, 1);
	}
}
