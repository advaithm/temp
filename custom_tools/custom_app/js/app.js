function Hammer() {
    var threads = document.getElementById("threads").value;
    if (threads) {
        if (isNaN(threads)) {
            document.getElementById("result").innerHTML = "Please enter number of threads"
        }
        else {
            var url = "http://localhost:8000/api/v1/hammer/" + threads;
            fetch(url)
                .then(response => response.json())
                .then(data => {
                    json = data.status
                    if (json.toLowerCase() === "success") {
                        document.getElementById("result").innerHTML = "Cpu stress test has begun"
                    }
                });
        }
    } else {
        document.getElementById("result").innerHTML = "Please enter number of threads"
    }
}

function end_Hammer() {
    var url = "http://localhost:8000/api/v1/hammer/kill"
    fetch(url)
        .then(response => response.json())
        .then(data => {
            json = data.status
            if (json.toLowerCase() === "killed") {
                document.getElementById("result").innerHTML = "Cpu stress test has ended"
            }
        });
}