// Execute with "gcc tests.c b-tree.c -o out -lm && ./out"
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <math.h>

#include "b-tree.h"

int log_base(int base, int value);
int sorted(int* values, int length);
int tree_leaves_have_correct_depth(BTreeNode* node, int depth, int best_case_depth, int worst_case_depth);
BTreeNode* create_tree_from_random_keys(int number_of_keys);
ArrayResult* shuffle(int* values, int length);

int main() {
    srand(time(0));


	printf("\n\nCreate trees and insert keys\n");
	printf("  Should return a tree with the correct depth\n");
	for(int i = 10; i <= 200; i += 10) {
		BTreeNode* tree = create_tree_from_random_keys(i);
		int best_case_depth = ceil(log_base(MAX_CHILDRENS, i + 1));
		int worst_case_depth = floor(log_base(MIN_CHILDRENS, (i + 1) / 2));
		printf("    %s",
			   tree_leaves_have_correct_depth(tree,
											  0,
											  best_case_depth,
											  worst_case_depth)
			   ? "\033[0;32m\u2714" : "\033[0;31m\u2717");
		printf(" Tree with %i keys must have a depth between %i and %i\n",
			   i, best_case_depth, worst_case_depth);
	}
	
	printf("\n  \033[0;37m"
		   "Should return values sorted after an inorder walk\n");
	for(int i = 10; i <= 200; i += 10) {
		BTreeNode* tree = create_tree_from_random_keys(i);
		ArrayResult* result = inorder(tree);
		printf("    %s",
			   sorted(result->values, result->length)
			   ? "\033[0;32m\u2714" : "\033[0;31m\u2717");
		printf(" Tree with %i values\n", i);
	}


	printf("\n\n\033[0;37mDelete keys\n");
	printf("\n  Delete all keys one by one\n"
		   "    Should at each step\n"
		   "      - return one value less when doing an inorder walk\n"
		   "      - return sorted values when doing an inorder walk\n");
	for(int i = 100; i <= 1000; i += 100) {
		BTreeNode* tree = create_tree_from_random_keys(i);
		ArrayResult* walk = inorder(tree);
		ArrayResult* permutation = shuffle(walk->values, walk->length);
		int test_length = TRUE;
		int test_sorted = TRUE;
		for(int j = 0; j < i; j++) {
			delete_key(tree, permutation->values[j]);
			walk = inorder(tree);
			if(walk->length != i - j - 1) {
				test_length = FALSE;
				break;
			}
			if(!sorted(walk->values, walk->length)) {
				test_sorted = FALSE;
				break;
			}
		}
		printf("    %s",
			   test_length && test_sorted
			   ? "\033[0;32m\u2714" : "\033[0;31m\u2717");
		printf(" Tree with %i values\n", i);
	}

  return 0;
}


// ------------------------ HELPER FUNCTIONS ------------------------

int sorted(int* values, int length) {
	for(int i = 0; i < length - 1; i++) {
		if(values[i] > values [i + 1]) return FALSE;
	}

	return TRUE;
}

int tree_leaves_have_correct_depth(BTreeNode* node, int depth, int max_depth, int min_depth) {
	if(node->leaf) {
		return depth <= min_depth && depth >= max_depth ? TRUE : FALSE;
	}
	
	int result = TRUE;
	for(int i = 0; i < node->number_of_keys + 1; i++) {
		result = result && tree_leaves_have_correct_depth(node->children[i], depth + 1, max_depth, min_depth);
	}

	return result;
}

BTreeNode* create_tree_from_keys(int* keys) {
	BTreeNode* root = b_tree_node_create();

	int i = -1;
    while(keys[++i] != -1) root = b_tree_insert(root, keys[i]);

	return root;
}

BTreeNode* create_tree_from_random_keys(int number_of_keys) {
	int keys[number_of_keys + 1];
	for(int i = 0; i < number_of_keys; i++) keys[i] = rand() % 1000;

	keys[number_of_keys] = -1;

	BTreeNode* root = create_tree_from_keys(keys);
	return root;
}

int log_base(int base, int value) {
	return log(value) / log(base);
}

ArrayResult* shuffle(int* values, int length) {
	ArrayResult* result = (ArrayResult*) malloc(sizeof(ArrayResult));
	result->values = (int*) malloc(length * sizeof(int));
	result->length = length;
	for(int i = 0; i < length; i++) result->values[i] = values[i];

	for (int i = length - 1; i >= 0; i--){
		int j = rand() % (i+1);

		int temp = result->values[i];
		result->values[i] = result->values[j];
		result->values[j] = temp;
	}

	return result;
}
