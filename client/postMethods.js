export async function postContent(title, content, date) {
  try {
    const response = await fetch("http://127.0.0.1:8081/create_post", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ title, content, date }),
      credentials: "include", //This might be unsafe :))
    });

    await response.json();
  } catch (error) {
    console.log("Error at posting content", error);
  }
}

export async function deletePost(post_id) {
  const objectId = post_id.$oid;
  try {
    const response = await fetch(
      `http://127.0.0.1:8081/delete_post/${objectId}`,
      {
        method: "DELETE",
        headers: { "Content-Type": "application/json" },
        credentials: "include", //This might be unsafe :))
      }
    );

    if (response.ok) {
      console.log(`Post with ID ${post_id} deleted successfully`);
    } else {
      console.error(`Failed to delete post with ID ${post_id}`);
    }
  } catch (error) {
    console.error("Error at deleting post:", error);
  }
}
export async function getAllPosts() {
  const response = await fetch("http://127.0.0.1:8081/posts");
  return await response.json();
}
export async function getPostById(post_id) {
  try {
    const response = await fetch(`http://127.0.0.1:8081/post/${post_id}`);
    if (response.ok) {
      const post = await response.json();
      return post;
    } else {
      console.error(`Failed to get post with ID ${post_id}`);
    }
  } catch (error) {
    console.error("Error at getting post:", error);
  }
}
