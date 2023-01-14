export default {
  install: (app) => {
    const files =import.meta.glob("./modules/*.js",{eager:true}) 

    for (const key in files) {
      if (Object.prototype.hasOwnProperty.call(files, key)) {
        app.use(files[key].default)
      }
  }
  },
}
