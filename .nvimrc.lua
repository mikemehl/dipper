local overseer = require('overseer')
local build_task = overseer.new_task({
  cmd = { 'cargo' },
  args = { 'build' },
  components = {
    { 'restart_on_save', delay = 500, interrupt = true },
    { 'unique' },
    'default'
  }
})

local run_task = overseer.new_task({
  cmd = { 'cargo' },
  args = { 'run' },
  components = {
    { 'unique' },
    'default'
  }
})
