import { createRouter, createWebHashHistory } from 'vue-router'
import AdvancedSolver from '../views/AdvancedSolver.vue';
import HelpView from '../views/HelpView.vue';
import PredView from '../views/DemandPredition.vue';
import Setting from '../views/SettingView.vue';
import PlanView from '../views/PlanView.vue';

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: AdvancedSolver
    },
    {
      path: '/help',
      name: 'help',
      component: HelpView
    },
    {
      path: '/pat/:share',
      name: 'pattern_share',
      component: PredView
    },
    {
      path: '/pred/:data',
      name: 'predition_data',
      component: PredView
    },
    {
      path: '/pred',
      name: 'predition',
      component: PredView
    },
    {
      path: '/plan',
      name: 'plan',
      component: PlanView
    },
    {
      path: '/plan/:share',
      name: 'plan_share',
      component: PlanView
    },
    {
      path: '/setting',
      name: 'setting',
      component: Setting
    }
  ]
})

export default router
