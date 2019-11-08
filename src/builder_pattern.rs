//! This mod is for demonstrating the builder pattern in Rust
//! # This module is for learning a pattern that's effective to model a workflow and is special because of Rust
//! ## Modelling a post publishing workflow
//! empty draft --add_content--> draft --review--> reviewed post --approve--> post
//!


///
/// For conveniently demonstrate the whole work flow
///
pub fn main_run()
{
    let mut post = Post::new();
    post.add_content("CONTENT!");
    let post = post.get_reviewed();
    let post = post.get_approved();
    post.get_printed();
}

///
/// This struct is the final product of the workflow
///
pub struct Post
{
    ///Notice the field is hidden
    ///
    /// So that it can not be directly constructed
    content:String
}

impl Post
{
    ///
    /// **Notice that the new function is returning a Draft struct and the field of Post is hidden**
    ///
    /// which enforce that the beginning the workflow is from Draft
    ///
    /// the Draft struct has limited methods
    ///
    /// which **assures** the direction the the workflow
    ///
    pub fn new() -> Draft
    {
        Draft
            {
                content:String::new()
            }
    }
}

///
/// This is the intermediate product of the workflow
///
/// Fields are also hidden to prevent direct constructions
///
/// which helps to assure the direction of the workflow
///
pub struct Draft
{
    content: String
}

impl Draft
{
    ///
    /// New contents can be added to draft
    ///
    pub fn add_content(&mut self, text:&str)
    {
        self.content.push_str(text);
    }

    ///
    /// To transform the draft to ReviewedDraft by using `self`
    ///
    /// which makes sense that once reviewed, no addition should be allowed
    ///
    pub fn get_reviewed(self) -> ReviewedDraft
    {
        ReviewedDraft
            {
                content : self.content
            }
    }
}

///
/// This is another intermediate product of the workflow
///
/// Fields are also hidden to prevent direct constructions
///
/// which helps to assure the direction of the workflow
///
pub struct ReviewedDraft
{
    content:String
}

impl ReviewedDraft
{
    ///
    /// To transform ReviewedDraft into the final product -- Post
    ///
    /// which makes sense that only approved draft can be post that get published
    pub fn get_approved(self) -> Post
    {
        Post
            {
                content:self.content
            }
    }
}

impl Post {
    ///
    /// To print posts
    ///
    /// which is allowed because it's been reviewed and approved
    ///
    pub fn get_printed(&self)
    {
        println!("PRINT: {}",self.content);
    }

    ///
    /// Drop a post
    ///
    /// Taking `self` and doing nothing
    ///
    pub fn drop(self){}

    ///
    /// Return contents
    ///
    /// which is allowed because it's been reviewed and approved
    ///
    pub fn content(&self)->&str
    {
        &self.content
    }
}

#[cfg(test)]
pub mod builder_pattern_test
{
    use super::*;
    #[test]
    fn test_run()
    {
        main_run();
    }
}