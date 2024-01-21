export function searchPosts() {
  const input = document.getElementById("search").value.toLowerCase();
  const postsList = document.getElementById("postsList");
  const searchList = document.getElementById("search-list");

  searchList.innerHTML = "";

  let titles = postsList.querySelectorAll("h4");

  titles.forEach((title) => {
    let searchValue = title.textContent.toLowerCase();

    if (searchValue.includes(input)) {
      const listItem = document.createElement("li");
      listItem.textContent = title.textContent;
      searchList.appendChild(listItem);
      listItem.style.display = "block";
    }
  });

  if (searchList.children.length > 0 && input !== "") {
    searchList.style.display = "block";
  } else {
    searchList.style.display = "none";
  }
}
