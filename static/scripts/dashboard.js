/**
 * @typedef Setting
 * @type {object}
 * @property {string} name - settings name
 * @property {bool} status - settings status
 */
const isDashboardEnabled = () => {
    fetch("/data/settings/enable-dashboard")
        .then(async (res) => {
            /** @type {Setting} */
            const data = await res.json()
            console.log(data)
            if (data.status) {
                updateDashboard()
            }
        })
        .catch((err) => {
            console.log(err)
        })
}

const updateDashboard = _ => {
    let dash = document.getElementById("dashboard-container")
    let p = document.createElement("p")
    p.innerText = "This is the Dashboard placeholder, feature under construction"
    dash.append(p)
}
