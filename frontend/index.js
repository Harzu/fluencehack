import * as fluence from "fluence";

const config = {
	appId: '42',
	contractAddress: '0xeFF91455de6D4CF57C141bD8bF819E5f873c1A01',
	ethUrl: "http://rinkeby.fluence.one:8545/"
}

let session = null
let targetUser = null
let users = []
const rooms = [
	'0x00',
	'0x01'
]

function getRooms() {
	return new Promise(resolve => {
		session.request(JSON.stringify({
			action: 'GetRooms',
			count: 10
		})).result()
			.then(res => JSON.parse(res.asString()))
			.then(res => resolve(res.rooms))
			.catch(error => console.error(error))
	})
}


function updUsers(rn) {

	session.request(JSON.stringify({
		action: 'getUsers',
		room_id: rn
	})).result()
	   .then(res => JSON.parse(res.asString()))
		 .then(res => {
			 console.log(res)
			 const uList = document.querySelector('.usr-list')
			 uList.innerHTML = ''
			 for (let player of res.players) {
				 const u = document.createElement('li')
				 u.className = 'usr-item'
				 u.textContent = player[0]
				 uList.appendChild(u)
			 }
		 })

	setTimeout(() => updUsers(), 3000)
}

async function connect() {
	try {
		session = await fluence.directConnect("localhost", 30000, 1, "session")
		console.log('connect estb')
	} catch (error) {
		console.error(error.message)
	}
}

window.addEventListener('DOMContentLoaded', async () => {
	const reqbtn = document.querySelector('.usr-info-upd')
	const findbtn = document.querySelector('.find')
	const createbtn = document.querySelector('.create')
	
	reqbtn.onclick = event => {
		event.preventDefault()
		const nameInp = document.querySelector('.inf-name-inp')
		const name = document.querySelector('.us-n')
		const val = nameInp.value
		name.textContent = 'your name: ' + val
		users.push(val)
		targetUser = val
		reqbtn.style.display = 'none'

		document.querySelector('.step-2').style.display = 'flex'

		connect()
	}

	findbtn.onclick = async event => {		
		event.preventDefault()
		const rList = document.querySelector('.found-room-l')
		const rooms = await getRooms()
		rList.innerHTML = ''
		for (let roomName of rooms) {
			const it = document.createElement('li')
			it.className = 'found-room-i'
			it.innerHTML = `<a href="#" class="roomLink" data-name="${roomName}">${roomName}</a>`
			rList.appendChild(it)

			document.querySelector('.found-rooms').style.display = 'block'
		}

		const rLink = document.querySelectorAll('.roomLink')
		for (let l of rLink) {
			l.onclick = e => {
				const target = e.target
				const rN = target.getAttribute('data-name')
				if (targetUser !== null) {
					session.request(JSON.stringify({
						action: 'Connect',
						room_id: rN,
						player_name: targetUser,
						army: [1,2,3,4,5,6,7,8,9,10]
					})).result()
						.then(res => JSON.parse(res.asString()))
						.then(res => {
							if (res.state === 'done') {
									document.querySelector('.room-name').textContent = rN
									const uList = document.querySelector('.usr-list')
									uList.innerHTML = ''
									const u = document.createElement('li')
									u.className = 'usr-item'
									u.textContent = targetUser
									uList.appendChild(u)
									updUsers(rN)		
							}
						})
				}

			}
		}
	}
	
	createbtn.onclick = event => {
		event.preventDefault()
		const rName = document.querySelector('.inp-room-name').value		
		if (targetUser !== null) {
			session.request(JSON.stringify({
				action: 'CreateBattle',
				room_id: rName,
				player_name: targetUser,
				army: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
			})).result()
				 .then(res => JSON.parse(res.asString()))
				 .then(res => {
					 if (res.state == 'done') {
						 document.querySelector('.room-name').textContent = rName
						 rooms.push(rName)
						 const uList = document.querySelector('.usr-list')
						 uList.innerHTML = ''
						 const u = document.createElement('li')
						 u.className = 'usr-item'
						 u.textContent = targetUser
						 uList.appendChild(u)
						 updUsers()
					 }
				 })
		}
	}
})
