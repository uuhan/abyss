package com.abyss.coral

import android.content.Context
import android.os.Bundle
import android.view.Menu
import android.util.Log
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.google.android.material.snackbar.Snackbar
import com.google.android.material.navigation.NavigationView
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import androidx.drawerlayout.widget.DrawerLayout
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.widget.Toolbar
import androidx.room.Room
import androidx.databinding.DataBindingUtil
import com.abyss.coral.ctp.Market
import com.abyss.coral.data.database.CoralDatabase
import com.abyss.coral.data.database.entities.Account
import com.facebook.stetho.Stetho
import timber.log.Timber

// data-bindings
import com.abyss.coral.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration

    override fun onCreate(state: Bundle?) {
        super.onCreate(state)
        DataBindingUtil.setContentView<ActivityMainBinding>(this, R.layout.activity_main)
        appContext = applicationContext

        // performance tool
        if (BuildConfig.DEBUG) {
            Timber.plant(Timber.DebugTree())
            Stetho.initializeWithDefaults(this)
        } else {
            Timber.plant(object : Timber.Tree() {
                // TODO: FirebaseCrashlytics
                override fun log(priority: Int, tag: String?, message: String, t: Throwable?) {
                    if (priority == Log.VERBOSE || priority == Log.DEBUG) {
                        return
                    }
                    if (priority == Log.ERROR) {
                        // FirebaseCrashlytics.getInstance().log("E/$tag:$message")
                    } else if (priority == Log.WARN) {
                        // FirebaseCrashlytics.getInstance().log("W/$tag:$message")
                    }
                }
            })
        }
        
        database = Room.databaseBuilder(this, CoralDatabase::class.java, DATABASE_NAME).build()

        val toolbar: Toolbar = findViewById(R.id.toolbar)
        setSupportActionBar(toolbar)

        val fab: FloatingActionButton = findViewById(R.id.fab)
        fab.setOnClickListener { view ->
            Snackbar.make(view, "Replace with your own action", Snackbar.LENGTH_LONG)
                    .setAction("Action", null).show()
        }
        val drawerLayout: DrawerLayout = findViewById(R.id.drawer_layout)
        val navView: NavigationView = findViewById(R.id.nav_view)
        val navController = findNavController(R.id.nav_host_fragment)
        // Passing each menu ID as a set of Ids because each
        // menu should be considered as top level destinations.
        appBarConfiguration = AppBarConfiguration(setOf(
                R.id.nav_home, R.id.nav_gallery, R.id.nav_slideshow), drawerLayout)
        setupActionBarWithNavController(navController, appBarConfiguration)
        navView.setupWithNavController(navController)

        val logDir = this.applicationContext.filesDir.absolutePath + "/"
        val market = Market(logDir, Account(brokerID="", userID="", password=""))
        Timber.i(market.tradingDay())
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.main, menu)
        return true
    }

    override fun onSupportNavigateUp(): Boolean {
        val navController = findNavController(R.id.nav_host_fragment)
        return navController.navigateUp(appBarConfiguration) || super.onSupportNavigateUp()
    }

    companion object {
        init {
            try {
                // 行情
                System.loadLibrary("clientencode")
                System.loadLibrary("thostmduserapi")
                System.loadLibrary("thostmduserapi_wrap")
                // 交易
                System.loadLibrary("thosttraderapi")
                System.loadLibrary("thosttraderapi_wrap")
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }

        private lateinit var appContext: Context
        private const val DATABASE_NAME = "coral.db"
        var database: CoralDatabase? = null

        @JvmStatic
        fun getAppContext(): Context {
            return appContext
        }
    }
}
