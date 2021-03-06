(function() {
    var firebaseConfig = {
        apiKey: "AIzaSyDkbJuTeQxrB7tVp6-iCSgp5FQz7ZVa-8o",
        authDomain: "unisulma-ti.firebaseapp.com",
        databaseURL: "https://unisulma-ti-default-rtdb.firebaseio.com",
        projectId: "unisulma-ti",
        storageBucket: "unisulma-ti.appspot.com",
        messagingSenderId: "36274356201",
        appId: "1:36274356201:web:7a730a99a0d4704ece9578",
        measurementId: "G-ZWKDEWREWB"
    };
    firebase.initializeApp(firebaseConfig);
    firebase.analytics();
})()

export function writeUserDataSetor(userId, setor) {
    firebase.database().ref('data/' + userId).update({
        setor: setor,
    });
    console.log("Write In Database - SETOR✅");
}
export function writeUserDataId(userId, id, ) {
    firebase.database().ref('data/' + userId).update({
        id: id,
    });
    console.log("Write In Database - ID✅");
}
export function writeUserDataHdd(userId, hdd) {
    firebase.database().ref('data/' + userId).update({
        hdd: hdd,
    });
    console.log("Write In Database - HDD✅");
}
export function writeUserDataCpu(userId, cpu) {
    firebase.database().ref('data/' + userId).update({
        cpu: cpu,
    });
    console.log("Write In Database✅ - CPU");
}
export function writeUserDataOs(userId, os) {
    firebase.database().ref('data/' + userId).update({
        os: os,
    });
    console.log("Write In Database - OPERATION SYSTEM✅");
}
export function writeUserDataUser(userId, user) {
    firebase.database().ref('data/' + userId).update({
        user: user,
    });
    console.log("Write In Database - USER✅");
}
export function writeUserDataMarca(userId, marca) {
    firebase.database().ref('data/' + userId).update({
        marca: marca,
    });
    console.log("Write In Database - MARCA✅");
}
export function writeUserDataMonitor(userId, monitor) {
    firebase.database().ref('data/' + userId).update({
        monitor: monitor,
    });
    console.log("Write In Database - MONITOR✅");
}
export function writeUserDataTamMonitor(userId, tamMonitor) {
    firebase.database().ref('data/' + userId).update({
        tamMonitor: tamMonitor,
    });
    console.log("Write In Database - TAM. MONITOR✅");
}
export function writeUserDataRam(userId, ram) {
    firebase.database().ref('data/' + userId).update({
        ram: ram,
    });
    console.log("Write In Database - RAM✅");
}
export function writeUserDataStatus(userId, status) {
    firebase.database().ref('data/' + userId).update({
        status: status
    });
    console.log("Write In Database - STATUS✅");
}
export function writeUserDataProblemas(userId, problemas, problemaId) {
    firebase.database().ref('data/' + userId).update({
        problemas: problemas
    });
    console.log("Write In Database - PROBLEMAS✅");
}
export function writeUserDataServicos(userId, servicos, servicoId) {
    firebase.database().ref('data/' + userId).update({
        servicos: servicos
    });
    console.log("Write In Database - SERVICOS✅");
}
export function writeUserDataVisibilidade(userId, visibilidade, user) {
    firebase.database().ref('data/' + userId).update({
        visibilidade: visibilidade,
        user: user,
    });
    console.log("Write In Database - VISIBILIDADE✅");
}

export function writeNewPc(setor, id, hdd, cpu, os, user, marca, monitor, tamMonitor, ram, status, servicos, problemas, new_push, visibilidade) {
    // A post entry.
    var postData = {
        setor: setor,
        id: id,
        hdd: hdd,
        cpu: cpu,
        os: os,
        user: user,
        marca: marca,
        monitor: monitor,
        tamMonitor: tamMonitor,
        ram: ram,
        status: status,
        problemas: problemas,
        servicos: servicos,
        visibilidade: visibilidade
    };

    // Get a key for a new Post.
    // var newPostKey = firebase.database().ref().child('data').push().len;
    // console.log(newPostKey);

    // Write the new post's data simultaneously in the posts list and the user's post list.
    var updates = {};
    updates['/data/' + new_push] = postData;
    // updates['/user-posts/' + uid + '/' + newPostKey] = postData;
    console.log("Write In Database - NEW POST✅");

    return firebase.database().ref().update(updates);
}

export function removeNewPc(setor, id, hdd, cpu, os, user, marca, monitor, tamMonitor, ram, status, servicos, problemas, new_push) {
    // A post entry.
    firebase.database().ref('data/' + new_push).update({
        setor: setor,
        id: id,
        hdd: hdd,
        cpu: cpu,
        os: os,
        user: user,
        marca: marca,
        monitor: monitor,
        tamMonitor: tamMonitor,
        ram: ram,
        status: status,
        problemas: problemas,
        servicos: servicos
    });

    // // Get a key for a new Post.
    // // var newPostKey = firebase.database().ref().child('data').push().len;
    // // console.log(newPostKey);

    // // Write the new post's data simultaneously in the posts list and the user's post list.
    // var updates = {};
    // updates['/data/' + new_push] = postData;
    // updates['/user-posts/' + uid + '/' + newPostKey] = postData;
    // firebase.database().ref('data' + new_push).set({
    //     status: status
    // });;
    console.log("Write In Database - REMOVE POST✅");
}