use crate::reader::Contents;

/// The interface representing the module responsible for evaluating the contents of a user-provided input file in accordance with the specifications 
/// of the challenge.
pub struct Evaluator<'a> {
    contents: &'a Contents
}

impl<'a> Evaluator<'a> {
    /// Creates a new [Evaluator] based on the provided [Contents].
    pub fn new(contents: &'a Contents) -> Self {
        Evaluator {
            contents
        }
    }

    /// Returns the cumulative sum of the values within the n (`groups_to_sum`) groups whose values have the highest sums within the contents of the input file.
    pub fn get_sorted_group_sum(&self, groups_to_sum: usize) -> i32 {
        // Create a new collection consisting of the summed values among each of the groups within the input contents
        let mut group_value_sums: Vec<i32> = self.contents.value_groups.iter().map(|group| group.iter().sum()).collect();

        // Sort the group sums in descending order
        group_value_sums.sort_by(|a, b| b.cmp(a));

        // Create a slice consisting of the requested number of groups (because the two challenges each call for different selections)
        let selected_group_values: &[i32] = &group_value_sums[..groups_to_sum];

        // Determine the sum of the selected groups
        let sum: i32 = selected_group_values.iter().sum::<i32>();

        // Return the determined sum
        sum
    }
}