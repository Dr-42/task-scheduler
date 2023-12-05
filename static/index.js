class Time {
    year;
    month;
    day;
    hour;
    minute;
    second;

    constructor(year, month, day, hour, minute, second) {
        this.year = year;
        this.month = month;
        this.day = day;
        this.hour = hour;
        this.minute = minute;
        this.second = second;
    }

    text() {
        return this.year + '-' + this.month + '-' + this.day + ' ' + this.hour + ':' + this.minute + ':' + this.second;
    }
}

class Task {
    constructor(id, name, status, parent_id, start_time, end_time, summary) {
        this.id = id;
        this.name = name;
        this.status = status;
        this.parent_id = parent_id;
        this.start_time = start_time;
        this.end_time = end_time;
        this.children = [];
        this.summary = summary;
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

    html() {
        let html = '<div class="task">';
        html += '<li>';
        html += '<div class="task-name">'
        html += '<div class="task-title">';
        if (this.children.length !== 0) {
            html += '<span class="caret" id="' + this.id + '">';
            html += this.icon() + ' ' + this.name;
            html += '</span>'
        } else {
            html += '<span class="empty-caret">';
            html += this.icon() + ' ' + this.name;
            html += '</span>'
        }
        if (this.end_time !== null) {
            html += '<div class="end-time">';
            html += 'Finished at: '
            html += this.end_time.text();
            html += '</div>';
        } else if (this.start_time !== null) {
            html += '<div class="start-time">';
            html += 'Started at: '
            html += this.start_time.text();
            html += '</div>';
        }
        html += '</div>';

        html += '<div class="side-buttons">'
        html += '<button onclick=rename_task(' + this.id + ')>✎</button>';
        if (this.status === 'InProgress') {
            html += '<button onclick=complete_task(' + this.id + ')>⇉</button>';
        } else if (this.status === 'Complete') {
            html += ' '
        } else if (this.status === 'Incomplete') {
            html += '<button onclick=start_task(' + this.id + ')>⇥</button>';
        }

        if (this.summary !== null) {
            html += '<button onclick=location.href="' + this.summary + '">📄</a>';
        }

        html += '<button onclick=add_child_task(' + this.id + ')>+</button>';
        html += '</div>';
        html += '</div>';
        if (this.children.length !== 0) {
            html += '<div class="task-children">';
            html += '<ul class="nested">';
            for (let i = 0; i < this.children.length; i++) {
                html += this.children[i].html();
            }
            html += '</ul>';
            html += '</div>';
        }
        html += '</li></div>';
        return html;
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
        let start_time = null;
        let end_time = null;
        if (task_data.start_time !== null) {
            start_time = new Time(task_data.start_time.year, task_data.start_time.month, task_data.start_time.day, task_data.start_time.hour, task_data.start_time.minute, task_data.start_time.second);
        }
        if (task_data.end_time !== null) {
            end_time = new Time(task_data.end_time.year, task_data.end_time.month, task_data.end_time.day, task_data.end_time.hour, task_data.end_time.minute, task_data.end_time.second);
        }
        let task = new Task(task_data.id, task_data.name, task_data.status, task_data.parent_id, start_time, end_time, task_data.summary);
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

function add_task() {
    let name = prompt("Task name:");
    if (name === null) {
        return;
    }
    // post
    fetch(`http://${global_ip}/addtask`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
        body: JSON.stringify({ name: name, parent: null })
    }).then(async data => {
        console.log(data);
        await reload();
    });
}


function add_child_task(parent_id) {
    let name = prompt("Task name:");
    if (name === null) {
        return;
    }
    // post
    fetch(`http://${global_ip}/addtask`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
        body: JSON.stringify({ name: name, parent: parent_id })
    }).then(async data => {
        console.log(data);
        await reload();
    });
}

function enable_toggles() {
    var toggler = document.getElementsByClassName("caret");
    var i;

    for (i = 0; i < toggler.length; i++) {
        toggler[i].addEventListener("click", function() {
            this.parentElement.parentElement.parentElement.querySelector(".nested").classList.toggle("active");
            this.classList.toggle("caret-down");
        });
    }
}

function restore_toggles(toggles) {
    for (let i = 0; i < toggles.length; i++) {
        let element = document.getElementById(toggles[i].toString());
        element.parentElement.parentElement.parentElement.querySelector(".nested").classList.add("active");
        element.classList.add("caret-down");
    }
}

function save_toggles(tasks) {
    let toggles = [];
    for (let i = 0; i < tasks.length; i++) {
        let task = tasks[i];
        let toggler = document.getElementById(task.id.toString());
        if (toggler !== null) {
            if (toggler.classList.contains("caret-down")) {
                toggles.push(task.id);
            }
        }
        if (task.children.length !== 0) {
            let child_toggles = save_toggles(task.children);
            toggles = toggles.concat(child_toggles);
        }
    }
    return toggles;
}

async function reload() {
    let toggles = save_toggles(global_tasks);
    // Wait 100 ms
    await new Promise(r => setTimeout(r, 100));
    fetch(`http://${global_ip}/tasks`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
    }).then(response => response.json())
        .then(data => {
            document.getElementById('task-list').innerHTML = '';
            global_tasks = parse_task_tree(data);
            for (let i = 0; i < global_tasks.length; i++) {
                let html = global_tasks[i].html();
                document.getElementById('task-list').innerHTML += html;
            }
            enable_toggles();
            restore_toggles(toggles);
            toggles = [];
        });
}

function start_task(task_id) {
    fetch(`http://${global_ip}/modifytask`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
        // body: JSON.stringify({name: name, parent_id: parent_id})
        body: JSON.stringify({ id: task_id, action: "start" })
    }).then(data => {
        console.log(data);
        reload();
    });
}

function dialogue_setup(summary_dialogue) {
    let nosum_button = document.getElementById('nosum');
    let submit_summary_button = document.getElementById('submit');

    nosum_button.onclick = function() {
        summary_dialogue.close('nosum');
    }

    submit_summary_button.onclick = function() {
        let summary_file = document.getElementById('summary-file').files[0];
        var reader = new FileReader;
        reader.readAsText(summary_file, "UTF-8");
        reader.onload = function(evt) {
            summary_dialogue.close(evt.target.result);
        }
        reader.onerror = function(evt) {
            summary_dialogue.close(null);
        }
    }
}

function complete_task(task_id) {
    // Dialogue to fetch the summary file. Can be blank
    let summary_dialogue = document.getElementById('summary-dialogue');
    summary_dialogue.showModal();
    dialogue_setup(summary_dialogue);

    summary_dialogue.addEventListener('close', function onClose() {
        console.log(summary_dialogue.returnValue);
        if (summary_dialogue.returnValue === 'nosum') {
            summary_path = null;
            fetch(`http://${global_ip}/modifytask`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                },
                // body: JSON.stringify({name: name, parent_id: parent_id})
                body: JSON.stringify({ id: task_id, action: "stop", summary: summary_path })
            }).then(async data => {
                console.log(data);
                await reload();
            });
        } else {
            summary_path = summary_dialogue.returnValue;
            fetch(`http://${global_ip}/modifytask`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                },
                // body: JSON.stringify({name: name, parent_id: parent_id})
                body: JSON.stringify({ id: task_id, action: "stop", summary: summary_path })
            }).then(async data => {
                console.log(data);
                await reload();
            });
        }
    });

}

function rename_task(task_id) {
    let name = prompt("New name:");
    if (name === null) {
        return;
    }
    // post
    fetch(`http://${global_ip}/renametask`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
        body: JSON.stringify({ id: task_id, name: name })
    }).then(async data => {
        console.log(data);
        await reload();
    });
}

window.onload = async function() {
    await reload();
};
