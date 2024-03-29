
export async function postContent(title, content, date) {
  const sanitizedData = purifyTitleAndContent(title,content);
  if (!sanitizedData){
    return;
  }
  try {
    const response = await fetch("http://127.0.0.1:8081/create_post", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ 
        title:sanitizedData.sanitizedTitle,
        content:sanitizedData.sanitizedContent,
        date }),
      credentials: "include",
    });
     await response.json();
    if (response.ok){
      document.getElementById("post-something").innerText = "Posted!"
    }
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
  } catch (error) {
    console.error("Error at deleting post:", error);
  }
}
export async function getAllPosts() {
    try {
      const response = await fetch("http://127.0.0.1:8081/posts");
      if (response.ok) {
        const post = await response.json();
        return post;
      } else {
        console.error(`Failed to get post`);
      }
    } catch (error) {
      console.error("Error at getting post:", error);
    }
}
export async function getPostById(post_id) {
  try {
    const response = await fetch(`http://127.0.0.1:8081/post/${post_id}`);
    if (response.ok) {
      const post = await response.json();
      return post;
    } else {
      console.error(`Failed to get post with ID`);
    }
  } catch (error) {
    console.error("Error at getting post:", error);
  }
}


function purifyTitleAndContent(title, content) {
  const sanitizedTitle = DOMPurify.sanitize(title);
  const sanitizedContent = DOMPurify.sanitize(content);
  if (sanitizedTitle === "" || sanitizedContent === "") {
    document.getElementById("post-error").innerText =
      "Invalid input, please try again";
    return null;
  }
  return { sanitizedTitle, sanitizedContent };
}

   export function sendPost(e) {
     e.preventDefault();
     grecaptcha.enterprise.ready(async () => {
       const token = await grecaptcha.enterprise.execute(
         "6LeFqVgpAAAAANzbXhYcFL9_9bKs6L9VAY0p6aVy",
         { action: "POST_CONTENT" }
       );
     });
   }