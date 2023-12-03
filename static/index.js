class Task {
    constructor(name, status, parent_id) {
        this.name = name;
        this.status = status;
        this.parent_id = parent_id;
        this.children = [];
    }

    add_child(child) {
        this.children.push(child);
    }

    icon() {
        if (this.status === 'Incomplete') {
            return '🔴';
        } else if (this.status === 'Complete') {
            return '🟢';
        } else if (this.status === 'InProgress') {
            return '🟡';
        }
    }
}

let global_tasks = [];

// Parse task tree
function parse_task_tree(task_datas) {
    // Convert a plain list of tasks into a tree
    // Each task has a parent_id, except the top level tasks

    // Create a map of tasks
    let task_map = {};
    let tasks = [];
    for (let i = 0; i < task_datas.length; i++) {
        let task_data = task_datas[i];
        let task = new Task(task_data.name, task_data.status, task_data.parent_id);
        task_map[task_data.id] = task;
        tasks.push(task);
    }

    // Create the task tree
    let task_tree = [];
    for (let i = 0; i < tasks.length; i++) {
        if (tasks[i].parent_id == null) {
            task_tree.push(tasks[i]);
        } else {
            let parent = task_map[tasks[i].parent_id];
            parent.add_child(tasks[i]);
        }
    }

    // Return the task tree
    return task_tree;
}

function task_html(task) {
    let html = '<li>' + task.icon() + ' ' + task.name;
    if (task.children != null) {
        html += '<ul>';
        for (let i = 0; i < task.children.length; i++) {
            html += task_html(task.children[i]);
        }
        html += '</ul>';
    }
    html += '</li>';
    return html;
}


window.onload = function () {
    // Allow CORS
    fetch('http://localhost:8080/tasks', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
    }).then(response => response.json())
        .then(data => {
            global_tasks = parse_task_tree(data);
            for (let i = 0; i < global_tasks.length; i++) {
                let html = task_html(global_tasks[i]);
                document.getElementById('task-list').innerHTML += html;
            }
        });
};